import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import type { TimerButton } from '../type'

export default function TimerButton({
    id,
    delay,
    default_text,
    running_text
}: TimerButton) {
    const [running, setRunning] = useState(false)
    const [timer, setTimer] = useState(0)

    const handleClick = () => {
        if (running) return
        invoke('handle_button', { id, action: true }).then(() => {
            setRunning(true)
            setTimer(delay)
        })
    }

    useEffect(() => {
        if (running) {
            const interval = setInterval(() => {
                setTimer(timer => {
                    if (timer > 1) {
                        return timer - 1
                    }
                    setRunning(false)

                    return timer
                })
            }, 1000)
            return () => clearInterval(interval)
        }
    }, [running])

    console.log(running, timer)

    return (
        <div style={{ margin: '2px', flexGrow: 1 }}>
            <button
                className={`btn btn-${running ? 'on btn-running' : 'off'}`}
                onClick={handleClick}
            >
                {running ? `${running_text} : ${timer}s` : default_text}
            </button>
        </div>
    )
}
