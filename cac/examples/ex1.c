float min(float x, float y)
{
    return x < y ? x : y;
}

float max(float x, float y)
{
    if (x > y) {
        return x;
    }

    return y;
}

int main(int argc, char *args[]) {
    int x = min(2.718, 3.141);

    return 0;
}
