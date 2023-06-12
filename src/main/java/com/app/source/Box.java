package com.app.source;

public final class Box {
    private final Pair x;
    private final Pair y;
    private final Pair z;

    public Box(Pair x, Pair y, Pair z) {
        this.x = x.ordered();
        this.y = y.ordered();
        this.z = z.ordered();
    }

    public Pair x() {
        return this.x;
    }

    public Pair y() {
        return this.y;
    }

    public Pair z() {
        return this.z;
    }

    Box wraps(Box other) {
        return new Box(Box.largerBound(this.x, other.x), Box.largerBound(this.y, other.y),
                Box.largerBound(this.z, other.z));
    }

    public Vector center() {
        return new Vector((this.x.x() + this.x.y()) / 2., (this.y.x() + this.y.y()) / 2.,
                (this.z.x() + this.z.y()) / 2.);
    }

    public Vector min() {
        return new Vector(x.x(), y.x(), z.x());
    }

    public Vector max() {
        return new Vector(x.y(), y.y(), z.y());
    }

    public boolean through(Vector source, Vector towards) {
        Vector minimum = min();
        Vector maximum = max();

        double tMin = Double.NEGATIVE_INFINITY;
        double tMax = Double.POSITIVE_INFINITY;

        for (int i = 0; i < 3; ++i) {
            double invB = 1. / towards.get(i);
            double tSmall = (minimum.get(i) - source.get(i)) * invB;
            double tLarge = (maximum.get(i) - source.get(i)) * invB;

            if (invB < 0) {
                double temp = tSmall;
                tSmall = tLarge;
                tLarge = temp;
            }

            assert tSmall <= tLarge;

            tMin = tSmall > tMin ? tSmall : tMin;
            tMax = tLarge < tMax ? tLarge : tMax;
        }

        return tMin < tMax;
    }

    static Pair largerBound(Pair a, Pair b) {
        return new Pair(a.x() < b.x() ? a.x() : b.x(), a.y() > b.y() ? a.y() : b.y());
    }

    public static Box wraps(Box a, Box b) {
        return a.wraps(b);
    }
}
