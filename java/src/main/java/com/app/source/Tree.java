package com.app.source;

import java.util.Collections;

public final class Tree implements Hittable {
    Hittable root;

    public Tree(com.app.source.List list) {
        this.root = Tree.recursivePartition(list.get());
    }

    private enum Axis {
        X, Y, Z
    }

    private static Axis maxVar(java.util.List<Hittable> list) {

        Vector avg = list.stream().map(Hittable::bounds).map(Box::center).reduce(new Vector(), Vector::add);

        Vector variance = list.stream().map(Hittable::bounds).map(Box::center).map(v -> v.sub(avg)).reduce(new Vector(),
                Vector::add);

        if (variance.x() > variance.y() && variance.x() > variance.z()) {
            return Axis.X;
        } else if (variance.y() > variance.z()) {
            return Axis.Y;
        } else {
            return Axis.Z;
        }
    }

    private static Hittable recursivePartition(java.util.List<Hittable> objects) {
        final var size = objects.size();

        switch (objects.size()) {
            case 0:
                assert false;
            case 1:
                return objects.get(0);
            case 2:
                return new TreeNode(objects.get(0), objects.get(1));
            default:
                switch (maxVar(objects)) {
                    case X:
                        Collections.sort(objects,
                                (a, b) -> Double.compare(a.bounds().center().x(), b.bounds().center().x()));
                        break;
                    case Y:
                        Collections.sort(objects,
                                (a, b) -> Double.compare(a.bounds().center().y(), b.bounds().center().y()));
                        break;
                    case Z:
                        Collections.sort(objects,
                                (a, b) -> Double.compare(a.bounds().center().z(), b.bounds().center().z()));
                        break;
                }

                var half = objects.size() / 2;

                var first = objects.subList(0, half);
                var last = objects.subList(half, size);

                return new TreeNode(recursivePartition(first), recursivePartition(last));
        }
    }

    @Override
    public HitData hit(Vector source, Vector towards) {
        return root.hit(source, towards);
    }

    @Override
    public Box bounds() {
        return root.bounds();
    }
}

class TreeNode implements Hittable {
    Box box;
    Hittable left;
    Hittable right;

    TreeNode(Hittable l, Hittable r) {
        box = Box.wraps(l.bounds(), r.bounds());
        left = l;
        right = r;
    }

    @Override
    public HitData hit(Vector source, Vector towards) {
        if (!box.through(source, towards)) {
            return HitData.miss();
        }

        var leftHit = left.hit(source, towards);
        var rightHit = right.hit(source, towards);

        boolean leftIsHit = leftHit.isHit();
        boolean rightIsHit = rightHit.isHit();

        return leftIsHit ? (rightIsHit ? (leftHit.t() < rightHit.t() ? leftHit : rightHit) : leftHit)
                : (rightIsHit ? rightHit : HitData.miss());
    }

    @Override
    public Box bounds() {
        return box;
    }
}
