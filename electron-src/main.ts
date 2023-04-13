import { app, BrowserWindow, dialog, ipcMain } from 'electron'
import * as nut from '@nut-tree/nut-js'
import * as path from 'path'

interface ToggleButton {
    id: string
    type: 'toggleButton' | 'aaaa'
    defaultState?: boolean
    enabledText: string
    disabledText: string
    enable: () => void
    disable: () => void
}

interface DelayButton {
    id: string
    type: 'delayButton'
    text: string
    delay: number
    action: () => void
}
type Button = ToggleButton | DelayButton

const buttons: Button[][] = [
    [
        {
            id: 'casino-fingerprints',
            type: 'toggleButton',
            enabledText: 'disable fingerprints (casino)',
            disabledText: 'enable fingerprints (casino)',
            enable: () => {
                console.log('enabling casino')
                subModulesState['casinoFingerprint'] = true
            },
            disable: () => {
                console.log('disabling casino')
                subModulesState['casinoFingerprint'] = false
            }
        },
        {
            id: 'cayo-fingerprints',
            type: 'toggleButton',
            enabledText: 'disable fingerprints (cayo)',
            disabledText: 'enable fingerprints (cayo)',
            enable: () => {
                console.log('enabling cayo')
                subModulesState['cayoFingerprint'] = true
            },
            disable: () => {
                console.log('disabling cayo')
                subModulesState['cayoFingerprint'] = false
            }
        }
    ],
    [],
    []
]

const subModulesState = {
    casinoFingerprint: false,
    cayoFingerprint: false
}

let win: BrowserWindow

function isResolutionSupported(width: number, height: number) {
    const supportedResolution = [[1920, 1080]]
    for (const res of supportedResolution) {
        if (res[0] == width && res[1] == height) {
            return true
        }
    }
    return false
}

async function createWindow() {
    win = new BrowserWindow({
        title: 'GTA 5 Assistant',
        width: 550,
        height: 200,
        //resizable: false,
        autoHideMenuBar: true,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        }
    })
    const width = await nut.screen.width()
    const height = await nut.screen.height()
    if (!isResolutionSupported(width, height)) {
        dialog.showErrorBox(
            'Resolution not supported',
            'The resolution of the screen is not currently supported\nBe sure to spam me on Discord : Kensa#4948'
        )
        process.exit(0)
    }
    if (app.isPackaged) {
        await win.loadFile(path.join(__dirname, '..', 'dist', 'index.html'))
    } else {
        win.loadURL('http://localhost:5173/')
    }
}

app.whenReady().then(createWindow)
if (app.isPackaged) {
    app.on('browser-window-created', (event, win) => {
        win.setMenu(null)
    })
}

ipcMain.on('getButtons', (event, args) => {
    event.returnValue = buttons.map(row =>
        row.map(btn => {
            switch (btn.type) {
                case 'delayButton':
                    return {
                        id: btn.id,
                        type: btn.type,
                        text: btn.text,
                        delay: btn.delay
                    }
                case 'toggleButton':
                    return {
                        id: btn.id,
                        type: btn.type,
                        enabledText: btn.enabledText,
                        disabledText: btn.disabledText
                    }
            }
        })
    )
})

ipcMain.on('toggleButton', (event, args) => {
    const id = args[0]
    const action = args[1]

    const button = buttons.flat().find(btn => btn.id == id)
    if (!button || button.type != 'toggleButton') {
        event.returnValue = false
        return
    }

    if (action) {
        button.enable()
    } else {
        button.disable()
    }
    event.returnValue = true
})
