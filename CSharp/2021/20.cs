using System;
using System.Collections;
using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2021
{
    public class Challenge20 : Challenge
    {
        private HashSet<Vector2Int> litPixels;
        private string enhanceAlgo;

        public override void ParseInput()
        {
            litPixels = new HashSet<Vector2Int>();
            enhanceAlgo = input[0];

            Vector2Int pixel = Vector3Int.Zero;
            for (pixel.y = 0; pixel.y < input.Length - 2; pixel.y++)
            {
                for (pixel.x = 0; pixel.x < input[3].Length; pixel.x++)
                {
                    if (input[2 + pixel.y][pixel.x] == '#')
                    {
                        litPixels.Add(pixel);
                    }
                }
            }
        }

        /* We are at that time of year where things require a little more
        explanation, so here we go.
        
        The basic concept here is that the image will grow by exactly one pixel
        in every direction for each enhancement process. This means I can keep
         track of the size of the image (the Rectangle bounds)
        and grow this every cycle.
        
        The image is stored as a set of vector2s indicating which pixels are lit.

        If the background is lit, a 2-pixel border is added to the image, but the
        bounds remain the same.

        Run the enhancement by checking every pixel in the bounds + 1. This will 
        allow the outside corners to read the border.

        Make sets of these checked pixels and combine with the image set to turn
        pixels on and off.

        The backgroundValue for the 2nd cycle is the first element in the
        enhancement algorithm because the pixels are all off in the background
        and add up to zero. If the first element is # that value the next time
        around will be the last element since the background adds up to a full
        9-bit int.

        Expand the bounds and the cycle is complete.
        */
        public override object Task1()
        {
            HashSet<Vector2Int> image = new HashSet<Vector2Int>();
            image.UnionWith(litPixels);

            Rectangle bounds = new Rectangle(0, 0, input[2].Length - 1, input.Length - 3);

            ENHANCE(2, image, bounds);
            return image.Count;
        }

        private void ENHANCE(int iters, HashSet<Vector2Int> image, Rectangle bounds)
        {

            bool backgroundValue = false;
            for (int i = 0; i < iters; i++)
            {

                // Add # border if required
                if (backgroundValue)
                {
                    for (int borderWidth = 1; borderWidth < 3; borderWidth++)
                    {
                        for (int y = bounds.yMin - borderWidth; y <= bounds.yMax + borderWidth; y++)
                        {
                            image.Add(new Vector2Int(bounds.xMin - borderWidth, y));
                            image.Add(new Vector2Int(bounds.xMax + borderWidth, y));
                        }

                        for (int x = bounds.xMin - borderWidth; x <= bounds.xMax + borderWidth; x++)
                        {
                            image.Add(new Vector2Int(x, bounds.yMin - borderWidth));
                            image.Add(new Vector2Int(x, bounds.yMax + borderWidth));
                        }
                    }
                }


                HashSet<Vector2Int> nextImage = new HashSet<Vector2Int>();

                Vector2Int scanRangeMin = new Vector2Int(bounds.xMin - 1, bounds.yMin - 1);
                Vector2Int scanRangeMax = new Vector2Int(bounds.xMax + 2, bounds.yMax + 2);
                Vector2Int pixel;
                for (pixel.x = bounds.xMin - 1; pixel.x < bounds.xMax + 2; pixel.x++)
                {
                    for (pixel.y = bounds.yMin - 1; pixel.y < bounds.yMax + 2; pixel.y++)
                    {
                        if (CalcNextPixel(pixel, image, bounds, backgroundValue))
                        {
                            nextImage.Add(pixel);
                        }
                    }
                }

                image.Clear();
                image.UnionWith(nextImage);

                bounds.xMin -= 1;
                bounds.xMax += 1;
                bounds.yMin -= 1;
                bounds.yMax += 1;

                char c = backgroundValue ? enhanceAlgo[^1] : enhanceAlgo[0];
                backgroundValue = c == '#';
            }
        }

        private bool CalcNextPixel(Vector2Int center, HashSet<Vector2Int> image, Rectangle bounds, bool backgroundValue)
        {
            int algoIndex = 0;
            Vector2Int neighbor;
            for (neighbor.y = center.y - 1; neighbor.y <= center.y + 1; neighbor.y++)
            {
                for (neighbor.x = center.x - 1; neighbor.x <= center.x + 1; neighbor.x++)
                {
                    {
                        algoIndex <<= 1;
                        algoIndex |= image.Contains(neighbor) ? 1 : 0;
                    }
                }
            }
            return enhanceAlgo[algoIndex] == '#';
        }

        public override object Task2()
        {
            HashSet<Vector2Int> image = new HashSet<Vector2Int>();
            image.UnionWith(litPixels);

            Rectangle bounds = new Rectangle(0, 0, input[2].Length - 1, input.Length - 3);

            ENHANCE(50, image, bounds);
            return image.Count;
        }
    }
}
