package com.app.source;

import java.util.concurrent.ThreadLocalRandom;
import java.util.stream.IntStream;

public final class Scene implements Hittable {
    private Vector source;
    private Vector corner;
    private Vector horizon;
    private Vector vertical;

    private Hittable list;
    private double aperture;

    public Scene(Vector source, Vector corner, Vector horizon, Vector vertical, double aperture) {
        this.source = source;
        this.corner = corner;
        this.horizon = horizon;
        this.vertical = vertical;
        this.list = null;
        this.aperture = aperture;
    }

    public Scene(Vector source, Vector corner, Vector horizon, Vector vertical, Hittable list, double aperture) {
        this.source = source;
        this.corner = corner;
        this.horizon = horizon;
        this.vertical = vertical;
        this.list = list;
        this.aperture = aperture;
    }

    public Vector source() {
        return this.source;
    }

    public void source(Vector v) {
        this.source = v;
    }

    public Vector corner() {
        return this.corner;
    }

    public void corner(Vector v) {
        this.corner = v;
    }

    public Vector horizon() {
        return this.horizon;
    }

    public void horizon(Vector v) {
        this.horizon = v;
    }

    public Vector vertical() {
        return this.vertical;
    }

    public void vertical(Vector v) {
        this.vertical = v;
    }

    public Hittable list() {
        return this.list;
    }

    public void list(Hittable a) {
        this.list = a;
    }

    public double aperture() {
        return this.aperture;
    }

    public void aperture(double a) {
        this.aperture = a;
    }

    public void save(Hittable h) {
        list = h;
    }

    public Vector color_trace(Vector starting, Vector towards, int depth) {
        var color = Vector.uniform(1.);
        for (int d = 0; d < depth; ++d) {
            var data = hit(starting, towards);

            if (data.isHit()) {
                var matter = data.matter();
                var reflected = matter.scatter(towards, data.normal());

                color.imul(matter.albedo());

                starting = data.point();
                towards = reflected;
            } else {
                double t = .5 * (towards.unit().y() + 1.);
                var back = Vector.uniform(1.).mul(1. - t).add(new Vector(.5, .7, 1.).mul(t));
                return color.mul(back);
            }
        }
        return Vector.o();
    }

    public int[] color(int x, int y, int ns, int depth, double dx, double dy) {
        var random = ThreadLocalRandom.current();

        double[] a = randomDisk(aperture);
        assert a.length == 2;
        double ai = a[0];
        double aj = a[1];

        Vector h = horizon.unit().mul(ai);
        Vector v = vertical.unit().mul(aj);
        Vector start = source.add(h).add(v);

        var color = IntStream.range(0, ns).sequential().mapToObj(_i -> {
            double i = ((double) x + random.nextDouble()) / dx;
            double j = ((double) y + random.nextDouble()) / dy;

            Vector end = corner.add(horizon.mul(i).add(vertical.mul(j)));
            Vector towards = end.sub(start);

            return color_trace(start, towards, depth);
        }).reduce(Vector.o(), Vector::add);

        var pixel = color.div(ns).mul(255.999);
        return new int[] { (int) pixel.x(), (int) pixel.y(), (int) pixel.z() };
    }

    public static double[] randomDisk(double radius) {
        var random = ThreadLocalRandom.current();
        for (;;) {
            double x = random.nextDouble();
            double y = random.nextDouble();

            if (x * x + y * y <= 1.) {
                return new double[] { x * radius, y * radius };
            }
        }
    }

    @Override
    public HitData hit(Vector source, Vector towards) {

        return list.hit(source, towards);
    }

    @Override
    public Box bounds() {
        return list.bounds();
    }
}
