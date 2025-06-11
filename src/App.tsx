import { useEffect, useState } from 'react'
import ToggleButton from './components/ToggleButton'
import TimerButton from './components/TimerButton'
import { invoke } from '@tauri-apps/api/core'
import { Button } from './type'

export default function App() {
    const [buttons, setButtons] = useState<JSX.Element[][]>([])
    useEffect(() => {
        invoke('get_buttons')
            .then(res => res as Button[][])
            .then(buttonRows => {
                const buttons = []
                let rowIndex = 0
                for (const row of buttonRows) {
                    const elementRow = []
                    let buttonIndex = 0
                    for (const button of row) {
                        const key = `${rowIndex}-${buttonIndex}`
                        if (button.type === 'toggle') {
                            elementRow.push(
                                <ToggleButton {...button} key={key} />
                            )
                        } else if (button.type === 'timer') {
                            elementRow.push(
                                <TimerButton {...button} key={key} />
                            )
                        }
                        buttonIndex++
                    }
                    buttons.push(elementRow)
                    rowIndex++
                }
                return buttons
            })
            .then(buttons => setButtons(buttons))
    }, [])

    return (
        <div id='app'>
            {buttons.map((row, i) => (
                <div key={i} className='btn-row'>
                    {...row}
                </div>
            ))}
        </div>
    )
}
