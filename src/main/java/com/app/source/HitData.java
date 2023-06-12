package com.app.source;

// for some reason I cannot enable preview features in maven
public final record HitData(double t, Vector point, Vector normal, Material matter) {
    public static HitData miss() {
        return new HitData(Double.POSITIVE_INFINITY, null, null, null);
    }

    public boolean isHit() {
        return t < Double.POSITIVE_INFINITY;
    }
}
