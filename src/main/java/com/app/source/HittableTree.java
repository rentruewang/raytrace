package com.app.source;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;

public final record HittableTree(Hittable root) implements Hittable {
    public HittableTree(HittableList list) {
        this(HittableTree.recursivePartition(list.objects()));
    }

    private enum Axis { X, Y, Z }

    private static Axis maxVar(java.util.List<Hittable> list) {
        Vector avg = list.stream()
                             .map(Hittable::bounds)
                             .map(Box::center)
                             .reduce(new Vector(), Vector::add);

        Vector variance = list.stream()
                                  .map(Hittable::bounds)
                                  .map(Box::center)
                                  .map(v -> v.sub(avg))
                                  .reduce(new Vector(), Vector::add);

        if (variance.x() > variance.y() && variance.x() > variance.z()) {
            return Axis.X;
        } else if (variance.y() > variance.z()) {
            return Axis.Y;
        } else {
            return Axis.Z;
        }
    }

    private static Hittable recursivePartition(List<Hittable> objects) {
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
                                (a, b)
                                        -> Double.compare(
                                                a.bounds().center().x(), b.bounds().center().x()));
                        break;
                    case Y:
                        Collections.sort(objects,
                                (a, b)
                                        -> Double.compare(
                                                a.bounds().center().y(), b.bounds().center().y()));
                        break;
                    case Z:
                        Collections.sort(objects,
                                (a, b)
                                        -> Double.compare(
                                                a.bounds().center().z(), b.bounds().center().z()));
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

record TreeNode(Box bounds, Hittable left, Hittable right) implements Hittable {
    TreeNode(Hittable l, Hittable r) {
        this(Box.wraps(l.bounds(), r.bounds()), l, r);
    }

    @Override
    public HitData hit(Vector source, Vector towards) {
        if (!bounds.through(source, towards)) {
            return HitData.miss();
        }

        var leftHit = left.hit(source, towards);
        var rightHit = right.hit(source, towards);

        boolean leftIsHit = leftHit.isHit();
        boolean rightIsHit = rightHit.isHit();

        switch (Arrays.toString(new boolean[] {leftIsHit, rightIsHit})) {
            case "[true, true]":
                return leftHit.t() < rightHit.t() ? leftHit : rightHit;
            case "[true, false]":
                return leftHit;
            case "[false, true]":
                return rightHit;
            case "[false, false]":
                return HitData.miss();
            default:
                throw new RuntimeException("unreachable");
        }
    }
}
