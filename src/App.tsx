import { useEffect, useState } from 'react'
import ToggleButton from './components/ToggleButton'
import TimerButton from './components/TimerButton'
import { invoke } from '@tauri-apps/api/core'
import type {
    Button as TButton,
    TimerButton as TTimerButton,
    ToggleButton as TToggleButton
} from './type'

export default function App() {
    const [buttons, setButtons] = useState<TButton[][]>([])

    useEffect(() => {
        invoke('get_buttons')
            .then(
                buttons =>
                    buttons as (
                        | { Toggle: Omit<TToggleButton, 'type'> }
                        | { Timer: Omit<TTimerButton, 'type'> }
                    )[][]
            )
            .then(buttons => {
                const newButtons: TButton[][] = []
                for (const row of buttons) {
                    const newRow: TButton[] = []
                    for (const button of row) {
                        if ('Toggle' in button) {
                            newRow.push({
                                type: 'toggleButton',
                                ...button.Toggle
                            })
                        } else if ('Timer' in button) {
                            newRow.push({
                                type: 'timerButton',
                                ...button.Timer
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

    return (
        <div id='app'>
            {buttons.map((row, i) => (
                <div key={i} className='btn-row'>
                    {row.map((btn, j) => {
                        switch (btn.type) {
                            case 'toggleButton':
                                return <ToggleButton key={j} {...btn} />
                            case 'timerButton':
                                return <TimerButton key={j} {...btn} />
                            default:
                                throw 'unknown button type'
                        }
                    })}
                </div>
            ))}
        </div>
    )
}
