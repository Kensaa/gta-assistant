export interface ToggleButton {
    id: string
    type: 'toggleButton'
    enabled_text: string
    disabled_text: string
}

export interface TimerButton {
    id: string
    type: 'timerButton'
    default_text: string
    running_text: string
    delay: number
}
export type Button = ToggleButton | TimerButton
