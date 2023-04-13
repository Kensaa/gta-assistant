import React, { PropsWithChildren, ReactElement, useState } from 'react'
import { ipcRenderer } from 'electron'
import Button from 'react-bootstrap/Button'

export interface ButtonProps {
    id: string
    defaultState?: boolean
    enabledText: string
    disabledText: string
}

export default function ToggleButton({
    id,
    defaultState = false,
    enabledText,
    disabledText
}: ButtonProps) {
    const [state, setState] = useState(defaultState)
    const handleClick = () => {
        console.log('clicked')
        ipcRenderer.send('toggleButton', [id, !state])
        setState(!state)
    }
    return (
        <div style={{ margin: '2px', flexGrow: 1 }}>
            <Button
                style={{ width: '100%' }}
                variant={state ? 'success' : 'primary'}
                onClick={handleClick}
            >
                {state ? enabledText : disabledText}
            </Button>
        </div>
    )
}
