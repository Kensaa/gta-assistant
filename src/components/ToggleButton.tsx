import React, { PropsWithChildren, ReactElement, useState } from 'react'

export interface ButtonProps {
    id: string
    defaultState?: boolean
    enabled_text: string
    disabled_text: string
}

export default function ToggleButton({
    id,
    defaultState = false,
    enabled_text,
    disabled_text
}: ButtonProps) {
    const [state, setState] = useState(defaultState)
    const handleClick = () => {
        console.log('clicked')
        // ipcRenderer.send('toggleButton', [id, !state])
        setState(!state)
    }
    return (
        <div style={{ margin: '2px', flexGrow: 1 }}>
            <Button
                style={{ width: '100%' }}
                variant={state ? 'success' : 'primary'}
                onClick={handleClick}
            >
                {state ? enabled_text : disabled_text}
            </Button>
        </div>
    )
}

function Button({
    children,
    onClick,
    style,
    variant
}: PropsWithChildren<{
    onClick: () => void
    style: React.CSSProperties
    variant: 'primary' | 'success'
}>) {
    return (
        <button
            style={style}
            className={`btn btn-${variant}`}
            onClick={onClick}
        >
            {children}
        </button>
    )
}
