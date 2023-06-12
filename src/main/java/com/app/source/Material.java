package com.app.source;

public interface Material {
    Vector scatter(Vector input, Vector normal);

    Vector albedo();
}
