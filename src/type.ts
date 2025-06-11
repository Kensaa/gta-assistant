export type Button = {
    id: string
} & (
    | {
          type: 'toggle'
          enabled_text: string
          disabled_text: string
      }
    | {
          type: 'timer'
          off_text: string
          on_text: string
          delay: number
      }
)
