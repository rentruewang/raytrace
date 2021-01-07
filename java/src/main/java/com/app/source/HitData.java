package com.app.source;

// for some reason I cannot enable preview features in maven
public final class HitData {
    private double t;
    private Vector point;
    private Vector normal;
    private Material matter;

    public HitData(double t, Vector point, Vector normal, Material matter) {
        this.t = t;
        this.point = point;
        this.normal = normal;
        this.matter = matter;
    }

    public double t() {
        return this.t;
    }

    public Vector point() {
        return this.point;
    }

    public Vector normal() {
        return this.normal;
    }

    public Material matter() {
        return this.matter;
    }

    public static HitData hit(double t, Vector point, Vector normal, Material matter) {
        return new HitData(t, point, normal, matter);
    }

    public static HitData miss() {
        return new HitData(Double.POSITIVE_INFINITY, null, null, null);
    }

    public boolean isHit() {
        return t != Double.POSITIVE_INFINITY;
    }
}
