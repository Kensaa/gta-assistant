import sharp from 'sharp'
import * as jimp from 'jimp'
import * as nut from '@nut-tree/nut-js'
import { Key, Region } from '@nut-tree/nut-js'

nut.keyboard.config.autoDelayMs = 20

/**
 * take a screenshot of a certain region
 * @param bound the bound of the region
 * @returns the screenshot
 */
export async function screen(bound: number[]) {
    const [x1, y1, x2, y2] = bound
    const rawScreen = await nut.screen.grabRegion(
        new Region(x1, y1, x2 - x1, y2 - y1)
    )
    const pngBuffer = await sharp(rawScreen.data, {
        raw: {
            width: rawScreen.width,
            height: rawScreen.height,
            channels: 4
        }
    })
        .png()
        .toBuffer()

    return jimp.read(pngBuffer)
}

/**
 * compute the difference between two images
 * @param img1 the first image
 * @param img2 the second image
 * @returns a number ranging from 0 to 1, 0 means they are believed to be identical
 */
export function imageSimilarity(img1: jimp, img2: jimp): number {
    return jimp.distance(img1, img2)
}

/**
 * returns a promise that resolves after a delay
 * @param delay the delay to wait
 * @returns a promise that resolve after the delay
 */
export async function wait(delay: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, delay))
}

/**
 * press a key
 * @param key the key to press
 */
export async function press(key: Key) {
    await nut.keyboard.pressKey(key)
    await nut.keyboard.releaseKey(key)
}

/**
 * store the element of the input array as the diffrence between the current element and the previous one
 * @param array input array
 * @returns output array
 */
export function relativeArray(array: number[]): number[] {
    let res: number[] = []
    let previous
    for (const e of array) {
        if (!previous) {
            res.push(e)
        } else {
            res.push(e - previous)
        }
        previous = e
    }
    return res
}
