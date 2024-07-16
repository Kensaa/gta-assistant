import * as jimp from 'jimp'
import { Key } from '@nut-tree-fork/nut-js'
import * as path from 'path'
import * as fs from 'fs'
import {
    screen,
    imageSimilarity,
    relativeArray,
    press,
    wait,
    minIndex,
    getScreenSize
} from './utils'

import {
    CASINO_FINGERPRINT_COUNT,
    CASINO_HEADER_POS,
    CASINO_FINGERPRINT_POS,
    CASINO_PARTS_POS,
    UPDATE_RATE
} from './constants'

/**
 * load the fill images of a number of fingerprints
 * @param count number of fingerprint
 * @returns an array of images
 */
async function loadFingerprints(
    count: number,
    height: number
): Promise<jimp[]> {
    return await Promise.all(
        new Array(count)
            .fill(0)
            .map((_, i) =>
                jimp.read(
                    path.join(
                        __dirname,
                        '..',
                        'assets',
                        height + '',
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
async function loadFingerprintParts(
    count: number,
    height: number
): Promise<jimp[][]> {
    let res: jimp[][] = []
    for (let index = 0; index < count; index++) {
        res.push(
            await Promise.all(
                new Array(4) // because there is 4 parts to check per fingerprint
                    .fill(0)
                    .map((_, i) =>
                        jimp.read(
                            path.join(
                                __dirname,
                                '..',
                                'assets',
                                height + '',
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
    const [_, height] = await getScreenSize()

    const HEADER_POS = CASINO_HEADER_POS[height]
    const FINGERPRINT_POS = CASINO_FINGERPRINT_POS[height]
    const PARTS_POS = CASINO_PARTS_POS[height]

    const headerIMG = await jimp.read(
        path.join(
            __dirname,
            '..',
            'assets',
            height + '',
            'casino',
            'header.png'
        )
    )

    const fingerprints = await loadFingerprints(
        CASINO_FINGERPRINT_COUNT,
        height
    )
    const fingerprintsParts = await loadFingerprintParts(
        CASINO_FINGERPRINT_COUNT,
        height
    )

    console.log('waiting for casino fingerprint ...')

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
    // await screenGrabber()
})()

async function screenGrabber() {
    const [width, height] = await getScreenSize()
    /*const headerIMG = await jimp.read(
        path.join(
            __dirname,
            '..',
            'assets',
            height + '',
            'casino',
            'header.png'
        )
    )*/
    const HEADER_POS = CASINO_HEADER_POS[height]
    const FINGERPRINT_POS = CASINO_FINGERPRINT_POS[height]
    const PARTS_POS = CASINO_PARTS_POS[height]

    const outPath = path.join(__dirname, '..', 'out', 'casinoScreenshots')

    let count = 1
    while (true) {
        ;(await screen([0, 0, width, height])).writeAsync(
            path.join(outPath, 'screen.png')
        )
        const currentOutputFolder = path.join(outPath, count + '')
        if (fs.existsSync(currentOutputFolder)) {
            count++
            continue
        }
        fs.mkdirSync(currentOutputFolder)

        const fingerprintScreen = await screen(FINGERPRINT_POS)
        await fingerprintScreen.writeAsync(
            path.join(currentOutputFolder, 'full.png')
        )

        const headerScreen = await screen(HEADER_POS)
        await headerScreen.writeAsync(
            path.join(currentOutputFolder, 'header.png')
        )

        for (let i = 0; i < PARTS_POS.length; i++) {
            const partScreen = await screen(PARTS_POS[i])
            await partScreen.writeAsync(
                path.join(currentOutputFolder, `${i + 1}.png`)
            )
        }

        count++
        await wait(2000)
    }
}
