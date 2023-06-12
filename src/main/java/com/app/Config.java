package com.app;

import java.util.concurrent.ThreadLocalRandom;

import com.app.source.*;;

public final class Config {
    public static final int PROGRESS = 1000;
    public static final int NX = 1200;
    public static final int NY = 675;
    public static final int NS = 100;
    public static final int DEP = 10;
    public static final double DEG = 30.;
    public static final double RATIO = (double) NX / (double) NY;
    public static final double APERTURE = 0.;
    public static final boolean TREE = true;

    static Material randomMaterial() {
        var random = ThreadLocalRandom.current();

        var mat = (int) (random.nextDouble() * 3.);
        assert 0 <= mat && mat < 3.;
        var blur = random.nextDouble() / 2.;
        assert 0 <= mat && mat < 1. / 2.;
        var refractive = random.nextDouble() + 1;
        assert 1. <= mat && mat < 2.;
        var albedo = Vector.random().add(1.).div(2.);

        switch (mat) {
            case 0:
                return new Matte(albedo);
            case 1:
                return new Metal(albedo, blur);
            case 2:
                return new Glass(albedo, blur, refractive);
            default:
        }
        assert false;
        return null;
    }

    public static Scene scenes() {
        var random = ThreadLocalRandom.current();

        var eye = new Vector(13., 2., 3.);
        var lookat = Vector.o();
        var viewup = Vector.j();

        var vision = lookat.sub(eye);

        var rad = Math.PI * DEG / 360.;

        var height = Math.tan(rad) * vision.length();
        var width = height * RATIO;

        var unit = vision.unit();
        var proj = unit.mul(viewup.dot(unit));
        viewup = viewup.sub(proj).unit();
        var horizon = vision.cross(viewup).unit();

        viewup.imul(height);
        horizon.imul(width);

        var list = new List();

        for (int i = -11; i < 11; ++i) {
            for (int j = -11; j < 11; ++j) {
                var iF = (double) i;
                var jF = (double) j;

                var center = new Vector(iF + .9 * random.nextDouble(), .2, jF + .9 * random.nextDouble());

                list.add(new Sphere(center, .2, randomMaterial()));
            }
        }

        list.add(new Sphere(new Vector(0., -1000., 0.), 1000., new Matte(Vector.uniform(.9))));

        list.add(new Sphere(Vector.j(), 1., new Glass(Vector.uniform(1.), 0., 1.5)));
        list.add(new Sphere(new Vector(-4., 1., 0.), 1., new Matte(new Vector(.4, .2, .1))));
        list.add(new Sphere(new Vector(4., 1., 0.), 1., new Metal(new Vector(.7, .6, .5), 0.)));

        var scene = new Scene(eye, lookat.sub(viewup).sub(horizon), horizon.mul(2.), viewup.mul(2.), APERTURE);

        var h = TREE ? new Tree(list) : list;

        scene.save(h);

        return scene;
    }
}
