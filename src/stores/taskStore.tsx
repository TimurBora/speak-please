import { create } from 'zustand'
import { commands } from '../bindings'
import type { UserQuestStatusResponse, SubmitProofRequest, SubmitProofResponse } from '../bindings'

interface TaskState {
  tasks: UserQuestStatusResponse[]
  isLoading: boolean
  error: string | null

  fetchTasks: () => Promise<void>
  submitProof: (
    questUlid: string,
    payload: SubmitProofRequest,
    images?: Uint8Array[],
    audios?: Uint8Array[]
  ) => Promise<SubmitProofResponse | null>

  getTaskById: (id: string) => UserQuestStatusResponse | undefined
}

export const useTaskStore = create<TaskState>((set, get) => ({
  tasks: [],
  isLoading: false,
  error: null,

  fetchTasks: async () => {
    set({ isLoading: true, error: null })
    try {
      const result = await commands.getDailyQuests()

      if (result.status === "ok") {
        set({ tasks: result.data, isLoading: false })
      } else {
        set({ error: result.error.message, isLoading: false })
      }
    } catch (err) {
      set({ error: "Failed to connect to backend", isLoading: false })
      console.error(err)
    }
  },

  submitProof: async (questUlid, payload, images, audios) => {
    set({ isLoading: true, error: null })
    try {
      const imageList = images ? images.map(img => Array.from(img)) : null
      const audioList = audios ? audios.map(aud => Array.from(aud)) : null

      const result = await commands.submitQuestProof(
        questUlid,
        payload,
        imageList,
        audioList
      )

      if (result.status === "ok") {
        await get().fetchTasks()
        set({ isLoading: false })
        return result.data
      } else {
        set({ error: result.error.message, isLoading: false })
        return null
      }
    } catch (err) {
      set({ error: "Submission failed", isLoading: false })
      return null
    }
  },

  getTaskById: (id) => {
    return get().tasks.find(t => t.quest.ulid === id)
  }
}))
