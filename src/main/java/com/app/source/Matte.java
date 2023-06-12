package com.app.source;

public final class Matte implements Material {
    private final Vector albedo;

    public Matte(Vector albedo) {
        this.albedo = albedo;
    }

    @Override
    public Vector scatter(Vector input, Vector normal) {
        normal = normal.unit();
        return Vector.randomBall(1.).add(normal);
    }

    @Override
    public Vector albedo() {
        return albedo;
    }
}
