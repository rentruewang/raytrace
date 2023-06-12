package com.app;

import com.app.source.Glass;
import com.app.source.Hittable;
import com.app.source.HittableList;
import com.app.source.HittableTree;
import com.app.source.Material;
import com.app.source.Matte;
import com.app.source.Metal;
import com.app.source.Scene;
import com.app.source.Sphere;
import com.app.source.Vector;

import java.lang.Math;
import java.util.concurrent.ThreadLocalRandom;

public final record Config(int width, int height, int samples, int depth, double degree,
        double aperture, boolean treeBased) {
    public static Config fromDefault() {
        return new Config(DefaultConfig.NX, DefaultConfig.NY, DefaultConfig.NS, DefaultConfig.DEP,
                DefaultConfig.DEG, DefaultConfig.APERTURE, DefaultConfig.TREE);
    }

    public Material randomMaterial() {
        var random = ThreadLocalRandom.current();

        var mat = (int) random.nextDouble() * 3;
        assert 0 <= mat && mat < 3;
        var blur = random.nextDouble() / 2.;
        assert 0 <= blur && blur < 1. / 2;
        var refrac = random.nextDouble() + 1;
        assert 1 <= refrac && refrac < 2;
        var albedo = Vector.random().add(1).div(2);

        switch (mat) {
            case 0:
                return new Matte(albedo);
            case 1:
                return new Metal(albedo, blur);
            case 2:
                return new Glass(albedo, blur, refrac);
            default:
                throw new RuntimeException("unreachable");
        }
    }

    public Scene scenes() {
        var random = ThreadLocalRandom.current();

        var eye = new Vector(13, 2, 3);
        var lookat = Vector.o();
        var viewup = Vector.j();

        var vision = lookat.sub(eye);

        var rad = Math.PI * degree / 360.;

        var height = Math.tan(rad) * vision.length();
        var ratio = (double) width / height;
        var width = height * ratio;

        var unit = vision.unit();
        var proj = unit.mul(viewup.dot(unit));
        viewup = viewup.sub(proj).unit();
        var horizon = vision.cross(viewup).unit();

        viewup = viewup.mul(height);
        horizon = horizon.mul(width);

        var list = new HittableList();

        for (int i = -11; i < 11; ++i) {
            for (int j = -11; j < 11; ++j) {
                var iF = (double) i;
                var jF = (double) j;

                var center = new Vector(
                        iF + .9 * random.nextDouble(), .2, jF + .9 * random.nextDouble());

                list.add(new Sphere(center, .2, randomMaterial()));
            }
        }

        list.add(new Sphere(new Vector(0, -1000, 0), 1000, new Matte(Vector.uniform(.9))));

        list.add(new Sphere(Vector.j(), 1, new Glass(Vector.uniform(1), 0, 1.5)));
        list.add(new Sphere(new Vector(-4, 1, 0), 1, new Matte(new Vector(.4, .2, .1))));
        list.add(new Sphere(new Vector(4, 1, 0), 1, new Metal(new Vector(.7, .6, .5), 0)));

        var scene = new Scene(
                eye, lookat.sub(viewup).sub(horizon), horizon.mul(2.), viewup.mul(2.), aperture);

        Hittable h = treeBased ? new HittableTree(list) : list;

        scene.save(h);

        return scene;
    }
}
