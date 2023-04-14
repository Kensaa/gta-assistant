import * as jimp from 'jimp'
import * as nut from '@nut-tree/nut-js'
import * as path from 'path'
import { screen, imageSimilarity, press, wait, minIndex } from './utils'

const UPDATE_RATE = 10
const HEADER_POS = [449, 60, 1661, 127]
const FINGERPRINT_POS = [907, 323, 1564, 976]
const FINGERPRINT_COUNT = 7

const PARTS_POS = [
    [415, 357, 820, 416],
    [415, 433, 820, 359],
    [415, 509, 820, 570],
    [415, 585, 820, 645],
    [415, 661, 820, 721],
    [415, 738, 820, 797],
    [415, 813, 820, 873],
    [415, 889, 820, 949]
]
/**
 * load the fill images of a number of fingerprints
 * @param count number of fingerprint
 * @returns an array of images
 */
async function loadFingerprints(count: number): Promise<jimp[]> {
    return await Promise.all(
        new Array(count)
            .fill(0)
            .map((_, i) =>
                jimp.read(
                    path.join(
                        __dirname,
                        '..',
                        'assets',
                        'cayo',
                        `${i + 1}`,
                        'full.png'
                    )
                )
            )
    )
}

/**
 * load the parts to check for a number of fingerprints
 * @param count number of fingerprint
 * @returns an array of array of images
 */
async function loadFingerprintParts(count: number): Promise<jimp[][]> {
    let res: jimp[][] = []
    for (let index = 0; index < count; index++) {
        res.push(
            await Promise.all(
                new Array(8)
                    .fill(0)
                    .map((_, i) =>
                        jimp.read(
                            path.join(
                                __dirname,
                                '..',
                                'assets',
                                'cayo',
                                `${index + 1}`,
                                `${i + 1}.png`
                            )
                        )
                    )
            )
        )
    }
    return res
}

;(async () => {
    const headerIMG = await jimp.read(
        path.join(__dirname, '..', 'assets', 'cayo', 'header.png')
    )

    const fingerprints = await loadFingerprints(FINGERPRINT_COUNT)
    const fingerprintsParts = await loadFingerprintParts(FINGERPRINT_COUNT)

    console.log('waiting for cayo fingerprint ...')

    while (true) {
        const headerScreenshot = await screen(HEADER_POS)
        if (imageSimilarity(headerScreenshot, headerIMG) < 0.1) {
            const fingerprintScreenshot = await screen(FINGERPRINT_POS)

            const similarities = fingerprints.map(e =>
                imageSimilarity(fingerprintScreenshot, e)
            )
            const fingerprintIndex = minIndex(similarities)
            console.log('fingerprint detected : ', fingerprintIndex + 1)

            for (let i = 0; i < 7; i++) {
                const current_pos = PARTS_POS[i]
                const currentPartScreen = await screen(current_pos)
                const partSimilarities = fingerprintsParts[
                    fingerprintIndex
                ].map(e => imageSimilarity(currentPartScreen, e))
                const currentPartIndex = minIndex(partSimilarities)

                const moveCount = currentPartIndex - i
                if (moveCount > 0) {
                    //move left
                    for (let j = 0; j < moveCount; j++) {
                        await press(nut.Key.Left)
                    }
                } else {
                    //move left
                    for (let j = 0; j < Math.abs(moveCount); j++) {
                        await press(nut.Key.Right)
                    }
                }
            }
            await wait(5000)
        }
        await wait(1000 / UPDATE_RATE)
    }
})()
