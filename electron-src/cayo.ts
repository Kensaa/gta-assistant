import * as jimp from 'jimp'
import * as nut from '@nut-tree/nut-js'
import * as path from 'path'
import * as fs from 'fs'
import { screen, imageSimilarity, press, wait, findImgInArray } from './utils'

const UPDATE_RATE = 10
const HEADER_POS = [449, 60, 1661, 127]
const FINGERPRINT_POS = [905, 321, 1565, 979]
const FINGERPRINT_COUNT = 7

const PARTS_POS = [
    [413, 357, 820, 417],
    [413, 433, 820, 493],
    [413, 509, 820, 569],
    [413, 585, 820, 645],
    [413, 661, 820, 721],
    [413, 737, 820, 797],
    [413, 813, 820, 873],
    [413, 889, 820, 949]
]

const MOVE_DELAY = 10
/**
 * load the full images of a number of fingerprints
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
                        'fingerprint.png'
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

    /*await wait(5000)

    await screenGraber()*/

    while (true) {
        const headerScreenshot = await screen(HEADER_POS)
        if (imageSimilarity(headerScreenshot, headerIMG) < 0.1) {
            const fingerprintScreenshot = await screen(FINGERPRINT_POS)

            const [_, fingerprintIndex] = findImgInArray(
                fingerprintScreenshot,
                fingerprints
            )

            console.log('fingerprint detected : ', fingerprintIndex + 1)

            const currentFingerprintParts = fingerprintsParts[fingerprintIndex]

            for (let i = 0; i < 8; i++) {
                const current_pos = PARTS_POS[i]
                const beforeScreen = new Date().getTime()
                const currentPartScreen = await screen(current_pos)
                const afterScreen = new Date().getTime()

                const [currentPartSimilarity, currentPartIndex] =
                    findImgInArray(currentPartScreen, currentFingerprintParts)

                const afterDistance = new Date().getTime()

                console.log('screen time', afterScreen - beforeScreen)
                console.log('distance time', afterDistance - afterScreen)
                console.log(
                    `part ${i + 1} : ${
                        currentPartIndex + 1
                    } with similarity ${currentPartSimilarity}`
                )

                const beforeMove = new Date().getTime()
                await moveTo(currentPartIndex, i)
                const afterMove = new Date().getTime()
                console.log('move time', afterMove - beforeMove)
                await press(nut.Key.Down)
                await wait(MOVE_DELAY)
                console.log('----')
            }
            console.log(
                '------------------------------------------------------------------------------'
            )
            await wait(2700)
        }
        await wait(1000 / UPDATE_RATE)
    }
})()

async function moveTo(current: number, target: number) {
    if (current === target) return

    if (target > current) {
        if (target - current > 4) {
            const moveCount = 8 - target + current
            console.log(`moving left ${moveCount} time(s)`)
            for (let j = 0; j < moveCount; j++) {
                await press(nut.Key.Left)
                await wait(MOVE_DELAY)
            }
        } else {
            const moveCount = target - current
            console.log(`moving right ${moveCount} time(s)`)
            for (let j = 0; j < moveCount; j++) {
                await press(nut.Key.Right)
                await wait(MOVE_DELAY)
            }
        }
    } else {
        if (target - current < -4) {
            const moveCount = 8 - current + target
            console.log(`moving right ${moveCount} time(s)`)
            for (let j = 0; j < moveCount; j++) {
                await press(nut.Key.Right)
                await wait(MOVE_DELAY)
            }
        } else {
            const moveCount = current - target
            console.log(`moving left ${moveCount} time(s)`)
            for (let j = 0; j < moveCount; j++) {
                await press(nut.Key.Left)
                await wait(MOVE_DELAY)
            }
        }
    }
}

async function screenGraber() {
    const outPath = path.join(__dirname, '..', 'out', 'cayoScreenshots')
    const headerIMG = await jimp.read(
        path.join(__dirname, '..', 'assets', 'cayo', 'header.png')
    )
    while (true) {
        const headerScreenshot = await screen(HEADER_POS)
        if (imageSimilarity(headerScreenshot, headerIMG) < 0.1) {
            const fullScreenshot = await screen([0, 0, 1920, 1080])
            const fingerprintScreenshot = await screen(FINGERPRINT_POS)

            // folder count in outPath
            const folders = fs
                .readdirSync(outPath)
                .filter(file =>
                    fs.statSync(path.join(outPath, file)).isDirectory()
                )

            let exists = false
            for (const folder of folders) {
                const folderPath = path.join(outPath, folder)
                if (!fs.existsSync(path.join(folderPath, 'fingerprint.png'))) {
                    continue
                }
                const full = await jimp.read(
                    path.join(folderPath, 'fingerprint.png')
                )
                const similarity = imageSimilarity(full, fingerprintScreenshot)
                if (similarity < 0.1) {
                    console.log('fingerprint already saved')
                    exists = true
                    break
                }
            }

            if (!exists) {
                const folderPath = path.join(outPath, `${folders.length + 1}`)
                fs.mkdirSync(folderPath)
                await fingerprintScreenshot.writeAsync(
                    path.join(folderPath, 'fingerprint.png')
                )
                await fullScreenshot.writeAsync(
                    path.join(folderPath, 'full.png')
                )

                // screenshot the parts
                for (let i = 0; i < 8; i++) {
                    const current_pos = PARTS_POS[0]
                    await press(nut.Key.Down)
                    await wait(50)
                    const currentPartScreen = await screen(current_pos)
                    await currentPartScreen.writeAsync(
                        path.join(folderPath, `${i + 1}.png`)
                    )
                    await press(nut.Key.Up)
                    await wait(50)
                    await press(nut.Key.Right)
                    await wait(50)
                }

                console.log('fingerprint saved')
            }
        }
        await wait(1000)
    }
}
