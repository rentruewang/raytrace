package com.app;

import java.io.File;
import java.nio.file.Paths;
import java.awt.Color;
import java.awt.image.BufferedImage;
import javax.imageio.ImageIO;
import javafx.util.Pair;

import java.util.stream.Collectors;
import java.util.stream.IntStream;
import me.tongfei.progressbar.ProgressBar;

public final class App {
    public static void main(String[] args) {
        var scene = Config.scenes();
        final int TOTAL = Config.NX * Config.NY;

        var bi = new BufferedImage(Config.NX, Config.NY, BufferedImage.TYPE_INT_RGB);

        var list = ProgressBar.wrap(IntStream.range(0, TOTAL), "Pixel processed").parallel().map(idx -> {
            var i = idx / Config.NY;
            var j = idx % Config.NY;

            var color = scene.color(i, j, Config.NS, Config.DEP, Config.NX, Config.NY);
            return new WithIndex(new Pair<>(i, j), color);
        }).collect(Collectors.toList());

        for (var data : list) {
            var pair = data.pair;
            var l = (int[]) data.list;

            var color = new Color(l[0], l[1], l[2]);
            bi.setRGB(pair.getKey(), Config.NY - pair.getValue() - 1, color.getRGB());
        }

        String folder = "images";
        String fname = "image.png";

        File dir = new File(folder);
        dir.mkdirs();

        var filepath = Paths.get(System.getProperty("user.dir"), folder, fname).toString();
        var file = new File(filepath);

        try {
            ImageIO.write(bi, "PNG", file);
        } catch (Exception e) {
        }

        System.out.println("Finished");
    }
}

class WithIndex {
    public Pair<Integer, Integer> pair;
    public int[] list;

    public WithIndex(Pair<Integer, Integer> pair, int[] list) {
        this.pair = pair;
        this.list = list;
    }
}
