table! {
    REGIONS (regionID) {
        regionID -> Nullable<Integer>,
        regionName -> Nullable<Text>,
    }
}

table! {
    RELEASES (releaseID) {
        releaseID -> Nullable<Integer>,
        romID -> Nullable<Integer>,
        releaseTitleName -> Nullable<Text>,
        regionLocalizedID -> Nullable<Integer>,
        TEMPregionLocalizedName -> Nullable<Text>,
        TEMPsystemShortName -> Nullable<Text>,
        TEMPsystemName -> Nullable<Text>,
        releaseCoverFront -> Nullable<Text>,
        releaseCoverBack -> Nullable<Text>,
        releaseCoverCart -> Nullable<Text>,
        releaseCoverDisc -> Nullable<Text>,
        releaseDescription -> Nullable<Text>,
        releaseDeveloper -> Nullable<Text>,
        releasePublisher -> Nullable<Text>,
        releaseGenre -> Nullable<Text>,
        releaseDate -> Nullable<Text>,
        releaseReferenceURL -> Nullable<Text>,
        releaseReferenceImageURL -> Nullable<Text>,
    }
}

table! {
    ROMs (romID) {
        romID -> Nullable<Integer>,
        systemID -> Nullable<Integer>,
        regionID -> Nullable<Integer>,
        romHashCRC -> Nullable<Text>,
        romHashMD5 -> Nullable<Text>,
        romHashSHA1 -> Nullable<Text>,
        romSize -> Nullable<Integer>,
        romFileName -> Nullable<Text>,
        romExtensionlessFileName -> Nullable<Text>,
        romParent -> Nullable<Text>,
        romSerial -> Nullable<Text>,
        romHeader -> Nullable<Text>,
        romLanguage -> Nullable<Text>,
        TEMPromRegion -> Nullable<Text>,
        romDumpSource -> Nullable<Text>,
    }
}

table! {
    SYSTEMS (systemID) {
        systemID -> Nullable<Integer>,
        systemName -> Nullable<Text>,
        systemShortName -> Nullable<Text>,
        systemHeaderSizeBytes -> Nullable<Integer>,
        systemHashless -> Nullable<Integer>,
        systemHeader -> Nullable<Integer>,
        systemSerial -> Nullable<Text>,
        systemOEID -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(REGIONS, RELEASES, ROMs, SYSTEMS,);

joinable!(RELEASES -> ROMs (romID));
