package com.app.source;

import java.util.concurrent.ThreadLocalRandom;

public final record Vector(double x, double y, double z) {
    public Vector() {
        this(0, 0, 0);
    }

    public double get(int i) {
        switch (i) {
            case 0:
                return x;
            case 1:
                return y;
            case 2:
                return z;
            default:
                throw new IllegalArgumentException(
                        String.format("Argument should be one of [0, 1, 2]. Received: %d", i));
        }
    }

    public Vector add(Vector other) {
        return new Vector(this.x + other.x, this.y + other.y, this.z + other.z);
    }

    public Vector add(double v) {
        return new Vector(this.x + v, this.y + v, this.z + v);
    }

    public Vector sub(Vector other) {
        return new Vector(this.x - other.x, this.y - other.y, this.z - other.z);
    }

    public Vector sub(double v) {
        return new Vector(this.x - v, this.y - v, this.z - v);
    }

    public Vector mul(Vector other) {
        return new Vector(this.x * other.x, this.y * other.y, this.z * other.z);
    }

    public Vector mul(double v) {
        return new Vector(this.x * v, this.y * v, this.z * v);
    }

    public Vector div(Vector other) {
        return new Vector(this.x / other.x, this.y / other.y, this.z / other.z);
    }

    public Vector div(double v) {
        return new Vector(this.x / v, this.y / v, this.z / v);
    }

    public double dot(Vector other) {
        return this.x * other.x + this.y * other.y + this.z * other.z;
    }

    public double l2() {
        return dot(this);
    }

    public double length() {
        return Math.sqrt(l2());
    }

    public Vector unit() {
        return div(length());
    }

    public Vector Sqrt() {
        return new Vector(Math.sqrt(x), Math.sqrt(y), Math.sqrt(z));
    }

    public boolean isNaN() {
        return Double.isNaN(x) || Double.isNaN(y) || Double.isNaN(z);
    }

    public Vector abs() {
        return new Vector(Math.abs(x), Math.abs(y), Math.abs(z));
    }

    public Vector cross(Vector other) {
        return new Vector(this.y * other.z - this.z * other.y, this.z * other.x - this.x * other.z,
                this.x * other.y - this.y * other.x);
    }

    @Override
    public String toString() {
        return String.format("Vector{%f, %f, %f}", x, y, z);
    }

    public static Vector uniform(double n) {
        return new Vector(n, n, n);
    }

    public static Vector o() {
        return Vector.uniform(0);
    }

    public static Vector i() {
        return new Vector(1, 0, 0);
    }

    public static Vector j() {
        return new Vector(0, 1, 0);
    }

    public static Vector k() {
        return new Vector(0, 0, 1);
    }

    public static Vector random() {
        var current = ThreadLocalRandom.current();
        return new Vector(current.nextDouble(), current.nextDouble(), current.nextDouble());
    }

    public static Vector randomBall(double radius) {
        for (;;) {
            var random = Vector.random();
            assert 0 <= random.x() && random.x() < 1;
            assert 0 <= random.y() && random.y() < 1;
            assert 0 <= random.z() && random.z() < 1;
            if (random.l2() <= 1.) {
                return random.mul(radius);
            }
        }
    }
}
