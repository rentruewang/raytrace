#include <fmt/ranges.h>

template <typename T, size_t S>
Vector<T, S>::Vector(const std::array<T, S>& vec) : vec_(vec) {}
template <typename T, size_t S>
Vector<T, S>::Vector(const Vector<T, S>& other) : vec_(other.vec_) {}

template <typename T, size_t S>
Vector<T, S> Vector<T, S>::random(void) {
    static thread_local std::mt19937 generator;
    std::uniform_real_distribution<T> distribution(0, 1);

    // Fixed size allocation on stack so this is ok.
    T arr[S];
    for (size_t i = 0; i < S; ++i) {
        T generated = distribution(generator);
        arr[i] = generated;
    }

    return Vector<T, S>(std::array(arr));
}

template <typename T, size_t S>
std::string Vector<T, S>::to_string(void) const {
    return fmt::format(FMT_STRING("{}"), vec_);
}
