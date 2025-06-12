import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Button } from '../type'

type Props = { type: 'toggle' } & Button
export default function ToggleButton({
    id,
    enabled_text,
    disabled_text,
    description
}: Props) {
    const [state, setState] = useState(false)
    const handleClick = () => {
        invoke('handle_button', { id, action: !state }).then(() =>
            setState(!state)
        )
    }
    return (
        <div style={{ margin: '2px', flexGrow: 1 }}>
            <button
                className={`btn btn-${state ? 'on' : 'off'}`}
                onClick={handleClick}
                title={description}
            >
                {state ? enabled_text : disabled_text}
            </button>
        </div>
    )
}
