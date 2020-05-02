#include <vector>
#include <set>
using namespace std;

extern "C"
{
    struct BinaryMatrix
    {
        int (*get)(struct BinaryMatrix *, int, int);
        int *(*dimensions)(struct BinaryMatrix *);
    };

    int leftMostColumnWithOne(struct BinaryMatrix *matrix)
    {
        int *d = matrix->dimensions(matrix);
        int n = d[0];
        int m = d[1];
        int j = m;
        while (j > 0)
        {
            int count = 0;
            for (int i = 0; i < n; i++)
            {
                while (j > 0 && matrix->get(matrix, i, j - 1) == 1)
                {
                    count++;
                    j--;
                }
            }
            if (count == 0)
            {
                break;
            }
        }
        return (j == m) ? -1 : j;
    }
}