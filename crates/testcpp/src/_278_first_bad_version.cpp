extern "C"
{
    bool isBadVersion(int version);

    int firstBadVersion(int n)
    {
        int lower = 1, upper = n, mid;
        while (lower < upper)
        {
            mid = lower + (upper - lower) / 2;
            if (!isBadVersion(mid))
                lower = mid + 1;
            else
                upper = mid;
        }
        return lower;
    }
}