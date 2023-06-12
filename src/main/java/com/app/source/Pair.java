package com.app.source;

public final class Pair {
    private final double x;
    private final double y;

    public Pair(double x, double y) {
        this.x = x;
        this.y = y;
    }

    public double x() {
        return this.x;
    }

    public double y() {
        return this.y;
    }

    Pair ordered() {
        return this.x < this.y ? this : new Pair(this.y, this.x);
    }
}
