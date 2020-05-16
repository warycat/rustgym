#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CString;

#[allow(dead_code)]
mod bindings;

use bindings::*;

type Attrs = Vec<(CString, CString, CString)>;

fn make_attrs(pairs: Vec<(&str, &str)>) -> Attrs {
    pairs
        .into_iter()
        .map(|p| {
            (
                CString::new(p.0.to_string()).expect("key"),
                CString::new(p.1.to_string()).expect("value"),
                CString::new("").expect("default"),
            )
        })
        .collect()
}

fn set_attrs(obj: *mut c_void, attrs: Attrs) {
    unsafe {
        for (k, v, d) in attrs {
            agsafeset(obj as *mut c_void, k.into_raw(), v.into_raw(), d.into_raw());
        }
    }
}

pub struct GSVG {
    gvc: *mut GVC_t,
    graph: *mut Agraph_t,
    nodes: Vec<*mut Agnode_t>,
    edges: Vec<*mut Agedge_t>,
}

impl GSVG {
    pub fn new(name: &str) -> Self {
        let g_attrs = make_attrs(vec![("label", name), ("rankdir", "BT")]);
        unsafe {
            let graph = agopen(
                g_attrs[0].clone().1.into_raw(),
                Agundirected,
                0 as *mut Agdisc_s,
            );
            set_attrs(graph as *mut c_void, g_attrs);
            GSVG {
                gvc: gvContext(),
                graph,
                nodes: vec![],
                edges: vec![],
            }
        }
    }

    pub fn node(&mut self, val: i32) {
        let n_attrs = make_attrs(vec![
            ("name", &val.to_string()),
            ("shape", "circle"),
            ("style", "filled"),
        ]);
        unsafe {
            let node = agnode(self.graph, n_attrs[0].clone().1.into_raw(), 1);
            set_attrs(node as *mut c_void, n_attrs);
            self.nodes.push(node);
        }
    }

    pub fn edge(&mut self, t: usize, h: usize, val: i32) {
        let e_attrs = make_attrs(vec![("name", &val.to_string()), ("dir", "back")]);
        unsafe {
            let e = agedge(
                self.graph,
                self.nodes[t],
                self.nodes[h],
                e_attrs[0].clone().1.into_raw(),
                1,
            );
            set_attrs(e as *mut c_void, e_attrs);
            self.edges.push(e);
        }
    }

    pub fn update_nodes(&self, n_attrs: Vec<HashMap<String, String>>) {
        assert_eq!(self.nodes.len(), n_attrs.len());
        for (i, attrs) in n_attrs.into_iter().enumerate() {
            for (key, value) in attrs {
                let key: CString = CString::new(key).expect("key");
                let value: CString = CString::new(value).expect("value");
                unsafe {
                    agsafeset(
                        self.nodes[i] as *mut c_void,
                        key.into_raw(),
                        value.into_raw(),
                        CString::new("").expect("default").into_raw(),
                    );
                }
            }
        }
    }

    pub fn render(&self, filename: &str) {
        unsafe {
            gvFreeLayout(self.gvc, self.graph); /* ignore errors */
            gvLayout(
                self.gvc,
                self.graph,
                CString::new("dot").expect("dot").into_raw(),
            );
            gvRenderFilename(
                self.gvc,
                self.graph,
                CString::new("svg").expect("svg").into_raw(),
                CString::new(filename).expect("svg").into_raw(),
            );
        }
    }
}

impl Drop for GSVG {
    fn drop(&mut self) {
        unsafe {
            gvFreeLayout(self.gvc, self.graph);
            agclose(self.graph);
            gvFreeContext(self.gvc);
        }
    }
}
