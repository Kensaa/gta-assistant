import React, { PropsWithChildren, useEffect, useState } from 'react'
import { ipcRenderer } from 'electron'
import Button from './components/ToggleButton'
import ToggleButton from './components/ToggleButton'

interface ToggleButton {
    id: string
    type: 'toggleButton'
    defaultState?: boolean
    enabledText: string
    disabledText: string
}

interface DelayButton {
    id: string
    type: 'delayButton'
    text: string
    delay: number
}
type Button = ToggleButton | DelayButton

export default function App() {
    const [buttons, setButtons] = useState<Button[][]>(
        ipcRenderer.sendSync('getButtons')
    )
    console.log(buttons)
    return (
        <div className='w-100 h-100 d-flex flex-column align-items-center p-2'>
            {buttons.map(row => (
                <ButtonRow>
                    {row.map(btn => {
                        switch (btn.type) {
                            case 'toggleButton':
                                return <ToggleButton {...btn} />
                            case 'delayButton':
                                return
                            default:
                                throw 'unknown button type'
                        }
                    })}
                </ButtonRow>
            ))}
        </div>
    )
}

function ButtonRow({ children }: PropsWithChildren) {
    return (
        <div className='w-100 d-flex flex-row align-item-center justify-content-center'>
            {children}
        </div>
    )
}
