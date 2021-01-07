package com.app.source;

import java.util.concurrent.ThreadLocalRandom;

public final class Glass implements Material {
    private final Vector albedo;
    private final double blur;
    private final double refractive;

    public Glass(Vector albedo, double blur, double refractive) {
        this.albedo = albedo;
        this.blur = blur;
        this.refractive = refractive;
    }

    public double blur() {
        return this.blur;
    }

    public double refractive() {
        return this.refractive;
    }

    static double schlick(double cosine, double ratio) {
        double r = (1. - ratio) / (1. + ratio);
        double sq = r * r;
        return sq + (1. - sq) * Math.pow(1. - cosine, 5);
    }

    @Override
    public Vector scatter(Vector input, Vector normal) {
        input = input.unit();
        normal = normal.unit();
        var cosine = input.dot(normal);

        double ratio = cosine < 0. ? 1. / refractive : refractive;

        double sineSq = 1. - cosine * cosine;
        double cosineSq = 1. - ratio * ratio * sineSq;
        boolean refract = cosine <= 0. || cosineSq >= 0.;

        var random = ThreadLocalRandom.current();

        double randDouble = random.nextDouble();
        Vector randBlur = Vector.randomBall(blur);

        if (refract && randDouble > schlick(Math.abs(cosine), refractive)) {
            var first = input.add(normal.mul(cosine));
            var second = normal.mul(Math.sqrt(cosineSq));

            return first.mul(ratio).sub(second).add(randBlur);
        }

        var casted = normal.mul(input.dot(normal) * 2.);
        return randBlur.add(input).sub(casted);

    }

    @Override
    public Vector albedo() {
        return albedo;
    }
}
