package com.app.source;

import java.util.ArrayList;

public final class List implements Hittable {
    private ArrayList<Hittable> objects;

    public List() {
        this.objects = new ArrayList<Hittable>();
    }

    public List(ArrayList<Hittable> objects) {
        this.objects = objects;
    }

    public ArrayList<Hittable> get() {
        return objects;
    }

    public void add(Hittable obj) {
        objects.add(obj);
    }

    @Override
    public HitData hit(Vector source, Vector towards) {
        var minHit = HitData.miss();
        for (Hittable h : objects) {
            var data = h.hit(source, towards);
            minHit = (data.t() < minHit.t()) ? data : minHit;
        }
        return minHit;
    }

    @Override
    public Box bounds() {
        switch (objects.size()) {
            case 0:
                assert false;
            case 1:
                return objects.get(0).bounds();
            default:
                var iter = objects.iterator();

                var bnd = iter.next().bounds();

                while (iter.hasNext()) {
                    bnd = Box.wraps(bnd, iter.next().bounds());
                }
                return bnd;
        }
    }

}
