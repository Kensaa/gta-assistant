import { PropsWithChildren, useEffect, useState } from 'react'
import Button from './components/ToggleButton'
import ToggleButton from './components/ToggleButton'
import { invoke } from '@tauri-apps/api/tauri'

interface ToggleButton {
    id: string
    type: 'toggleButton'
    defaultState?: boolean
    enabled_text: string
    disabled_text: string
}

interface DelayButton {
    id: string
    type: 'delayButton'
    default_text: string
    running_text: string
    delay: number
}
type Button = ToggleButton | DelayButton

export default function App() {
    const [buttons, setButtons] = useState<Button[][]>([])

    useEffect(() => {
        invoke('get_buttons')
            .then(
                buttons =>
                    buttons as (
                        | { Toggle: Omit<ToggleButton, 'type'> }
                        | { Delay: Omit<DelayButton, 'type'> }
                    )[][]
            )
            .then(buttons => {
                const newButtons: Button[][] = []
                for (const row of buttons) {
                    console.log('row', row)
                    const newRow: Button[] = []
                    for (const button of row) {
                        console.log('button', button)
                        if ('Toggle' in button) {
                            newRow.push({
                                type: 'toggleButton',
                                ...button.Toggle
                            })
                        } else if ('Delay' in button) {
                            newRow.push({
                                type: 'delayButton',
                                ...button.Delay
                            })
                        }
                    }
                    newButtons.push(newRow)
                }
                console.log(buttons)
                console.log(newButtons)
                setButtons(newButtons)
            })
    }, [])

    // console.log(buttons)
    // console.log(JSON.stringify(buttons))
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
