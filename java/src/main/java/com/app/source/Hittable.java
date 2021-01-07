package com.app.source;

public interface Hittable {
    HitData hit(Vector source, Vector towards);

    Box bounds();
}
