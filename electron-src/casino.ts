import * as jimp from 'jimp'
import * as nut from '@nut-tree/nut-js'
import { Key } from '@nut-tree/nut-js'
import * as path from 'path'
import { screen, imageSimilarity, relativeArray, press, wait } from './utils'

const UPDATE_RATE = 10
const FINGERPRINT_COUNT = 4
const HEADER_POS = [370, 90, 1550, 120]
const FINGERPRINT_POS = [974, 157, 1320, 685]

const PARTS_POS = [
    [475, 271, 595, 391],
    [618, 271, 738, 391],
    [475, 414, 595, 535],
    [618, 414, 738, 535],
    [475, 558, 595, 680],
    [618, 558, 738, 680],
    [475, 702, 595, 823],
    [618, 702, 738, 823]
]

/**
 * returns the index of the smallest element of the array
 * @param array the input array
 * @returns the index
 */
function minIndex(array: number[]): number {
    let min = 0
    for (let i = 0; i < array.length; i++) {
        if (array[i] < array[min]) min = i
    }
    return min
}

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
                        'casino',
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
                new Array(4) // beacause there is 4 parts to check per fingerprint
                    .fill(0)
                    .map((_, i) =>
                        jimp.read(
                            path.join(
                                __dirname,
                                '..',
                                'assets',
                                'casino',
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
        path.join(__dirname, '..', 'assets', 'casino', 'header.png')
    )

    const fingerprints = await loadFingerprints(FINGERPRINT_COUNT)
    const fingerprintsParts = await loadFingerprintParts(FINGERPRINT_COUNT)

    console.log('waiting for fingerprint ...')

    while (true) {
        const headerScreenshot = await screen(HEADER_POS)
        // check for hacking
        if (imageSimilarity(headerScreenshot, headerIMG) < 0.1) {
            // screen the fingerprint on the right
            const fingerprintScreenshot = await screen(FINGERPRINT_POS)
            // compare it with all known fingerprints
            const similarities = fingerprints.map(e =>
                imageSimilarity(fingerprintScreenshot, e)
            )
            // get the index of the most similar
            const fingerprintIndex = minIndex(similarities)
            // get all the parts to check on the left
            const solutions = fingerprintsParts[fingerprintIndex]
            console.log('fingerprint detected : ', fingerprintIndex + 1)
            // screen all parts on the left
            const parts_screenshots = await Promise.all(
                PARTS_POS.map(e => screen(e))
            )

            // get the position of the solutions in all the parts on the left
            // (this is because parts position are randomized)
            const positions: number[] = []
            for (const solution of solutions) {
                let i = 0
                let minI = 0
                let minV = 1
                for (const screenshot of parts_screenshots) {
                    if (!positions.includes(i)) {
                        const s = imageSimilarity(solution, screenshot)
                        if (s < minV) {
                            minV = s
                            minI = i
                        }
                    }
                    i++
                }
                positions.push(minI)
            }
            // we sort those position to get them in order, making the movements easier
            positions.sort()
            // store position as the numberof moves from the previous element to again make the movements easier
            const relativePositions = relativeArray(positions)

            // press the keys
            for (const move of relativePositions) {
                for (let i = 0; i < move; i++) {
                    await press(Key.Right)
                }
                await press(Key.Enter)
            }
            await press(Key.Tab)
            console.log('validating')
            await wait(4350 - 1000 / UPDATE_RATE)
        }
        await wait(1000 / UPDATE_RATE)
    }
})()
