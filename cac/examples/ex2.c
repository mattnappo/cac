struct vec3 {
    float x, y, z;
    float second;
    float zzz;
};

float norm(struct vec3 vec)
{
    return vec.x * vec.x + vec.y * vec.y + vec.z * vec.z;
}
