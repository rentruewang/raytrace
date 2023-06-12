package com.app.source;

public final record Pair(double x, double y) {
    Pair ordered() {
        return this.x <= this.y ? this : new Pair(this.y, this.x);
    }
}
