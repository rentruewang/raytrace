#pragma once
#include <stddef.h>
#include <array>
#include <iostream>
#include <random>
#include <string>

template <typename T, size_t S>
class Vector {
   public:
    Vector(const std::array<T, S>& vec);
    Vector(const Vector<T, S>& other);
    Vector(Vector<T, S>&& other) = delete;

    T operator[](size_t index) const;

    bool operator==(const Vector<T, S>& other) const;

    Vector<T, S> operator+(const Vector<T, S>& other) const;
    Vector<T, S> operator-(const Vector<T, S>& other) const;
    Vector<T, S> operator*(const Vector<T, S>& other) const;
    Vector<T, S> operator/(const Vector<T, S>& other) const;

    void operator+=(const Vector<T, S>& other);
    void operator-=(const Vector<T, S>& other);
    void operator*=(const Vector<T, S>& other);
    void operator/=(const Vector<T, S>& other);

    std::string to_string(void) const;

    Vector<T, S> dot(const Vector<T, S>& other) const;
    Vector<T, S> cross(const Vector<T, S>& other) const;

    static Vector<T, S> random(void);

   private:
    std::array<T, S> vec_;
};

template <typename T, size_t S>
std::ostream& operator<<(std::ostream& out, const Vector<T, S>& self);

#include "vectors.txx"
