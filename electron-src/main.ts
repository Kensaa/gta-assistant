import { app, BrowserWindow, ipcMain, dialog } from 'electron'
import * as path from 'path'

let win: BrowserWindow | null = null

async function createWindow() {
    win = new BrowserWindow({
        title: 'GTA 5 Assistant',
        width: 700,
        height: 700,
        autoHideMenuBar: true,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        }
    })

    if (app.isPackaged) {
        await win.loadFile(path.join(__dirname, '..', 'dist', 'index.html'))
    } else {
        win.loadURL('http://localhost:5173/')
    }
}

app.whenReady().then(createWindow)
if (app.isPackaged) {
    app.on('browser-window-created', function (event, win) {
        win.setMenu(null)
    })
}
