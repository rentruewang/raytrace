package com.app.source;

public final class Sphere implements Hittable {
    private Vector center;
    private double radius;
    private Material matter;

    public Sphere(Vector center, double radius, Material matter) {
        this.center = center;
        this.radius = radius;
        this.matter = matter;
    }

    public Vector center() {
        return this.center;
    }

    public void center(Vector v) {
        this.center = v;
    }

    public double radius() {
        return this.radius;
    }

    public void radius(double r) {
        this.radius = r;
    }

    public Material matter() {
        return this.matter;
    }

    public void matter(Material m) {
        this.matter = m;
    }

    public Vector normal(Vector point) {
        return point.sub(center);
    }

    @Override
    public HitData hit(Vector source, Vector towards) {
        Vector oc = normal(source);
        double a = towards.l2();
        double b = oc.dot(towards);
        double c = oc.l2() - radius * radius;

        double base = Math.sqrt(b * b - a * c);
        double neg = (-b - base) / a;
        double pos = (-b + base) / a;

        if (neg > 0) {
            Vector point = source.add(towards.mul(neg));
            assert bounds().through(source, towards);
            return HitData.hit(neg, point, normal(point), matter);
        } else if (pos > 0) {
            Vector point = source.add(towards.mul(pos));
            assert bounds().through(source, towards);
            return HitData.hit(pos, point, normal(point), matter);
        } else {
            return HitData.miss();
        }
    }

    @Override
    public Box bounds() {
        Vector min = center.sub(radius);
        Vector max = center.add(radius);

        return new Box(new Pair(min.x(), max.x()), new Pair(min.y(), max.y()), new Pair(min.z(), max.z()));
    }
}
