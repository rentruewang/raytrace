package com.app.source;

public final class Metal implements Material {
    private final Vector albedo;
    private final double blur;

    public Metal(Vector albedo, double blur) {
        this.albedo = albedo;
        this.blur = blur;
    }

    public double blur() {
        return this.blur;
    }

    @Override
    public Vector scatter(Vector input, Vector normal) {
        input = input.unit();
        normal = normal.unit();
        var random = Vector.o();
        // var random = Vector.randomBall(blur);
        var casted = normal.mul(input.dot(normal) * 2.);
        return random.add(input).sub(casted);
    }

    @Override
    public Vector albedo() {
        return this.albedo;
    }
}
