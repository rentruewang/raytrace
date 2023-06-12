package com.app;

import java.awt.Color;
import java.awt.image.BufferedImage;
import java.io.File;
import java.nio.file.Paths;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import javafx.util.Pair;

import javax.imageio.ImageIO;

import me.tongfei.progressbar.ProgressBar;

public final class App {
    public static void main(String[] args) {
        final var config = Config.fromDefault();
        final var scene = config.scenes();
        final int pixels = config.width() * config.height();

        var bi = new BufferedImage(config.width(), config.height(), BufferedImage.TYPE_INT_RGB);

        var list = ProgressBar.wrap(IntStream.range(0, pixels), "Percentage of pixels processed")
                           .parallel()
                           .map((Integer idx) -> {
                               var i = idx / config.width();
                               var j = idx % config.height();

                               var color = scene.color(i, j, config.samples(), config.depth(),
                                       config.width(), config.height());
                               return new WithIndex(new Pair<>(i, j), color);
                           })
                           .collect(Collectors.toList());

        for (var data : list) {
            var x = data.pair.getKey();
            var y = data.pair.getValue();

            assert x >= 0 && x < config.width();
            assert y >= 0 && y < config.height();

            var l = data.list;

            var rgb = new Color(l[0], l[1], l[2]).getRGB();

            assert bi.getWidth() == config.width();
            assert bi.getHeight() == config.height();
            bi.setRGB(x, config.height() - y - 1, rgb);
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
            System.out.println("Don't want to handle this.");
        }
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
