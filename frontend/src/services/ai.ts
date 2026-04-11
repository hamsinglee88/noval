import axios from 'axios'
import { config } from '@/config'

export interface AIRequest {
  action: 'continue' | 'polish' | 'expand' | 'summarize' | 'rewrite'
  text: string
  context?: string
  styleId?: string
}

export interface AIResponse {
  success: boolean
  result: string
  usage?: {
    promptTokens: number
    completionTokens: number
    totalTokens: number
  }
}

export interface AIStreamChunk {
  type: 'token' | 'done' | 'error'
  content?: string
  error?: string
}

const api = axios.create({
  baseURL: '/api/ai',
  timeout: 60000
})

// 添加认证拦截器
api.interceptors.request.use(config => {
  const session = localStorage.getItem('noval-auth-session')
  if (session) {
    try {
      const CryptoJS = require('crypto-js')
      const decrypted = CryptoJS.AES.decrypt(session, config.encryptionKey).toString(CryptoJS.enc.Utf8)
      const data = JSON.parse(decrypted)
      config.headers.Authorization = `Bearer ${data.token}`
    } catch (e) {
      console.error('Failed to decrypt session:', e)
    }
  }
  return config
})

export const aiApi = {
  // 续写
  async continue(request: AIRequest): Promise<AIResponse> {
    const response = await api.post('/continue', request)
    return response.data
  },

  // 润色
  async polish(request: AIRequest): Promise<AIResponse> {
    const response = await api.post('/polish', request)
    return response.data
  },

  // 扩写
  async expand(request: AIRequest): Promise<AIResponse> {
    const response = await api.post('/expand', request)
    return response.data
  },

  // 总结
  async summarize(request: AIRequest): Promise<AIResponse> {
    const response = await api.post('/summarize', request)
    return response.data
  },

  // 改写
  async rewrite(request: AIRequest): Promise<AIResponse> {
    const response = await api.post('/rewrite', request)
    return response.data
  },

  // 流式生成
  async stream(request: AIRequest, onChunk: (chunk: AIStreamChunk) => void): Promise<void> {
    const response = await fetch('/api/ai/stream', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${getToken()}`
      },
      body: JSON.stringify(request)
    })

    if (!response.ok) {
      throw new Error('AI request failed')
    }

    const reader = response.body?.getReader()
    if (!reader) {
      throw new Error('No response body')
    }

    const decoder = new TextDecoder()
    
    while (true) {
      const { done, value } = await reader.read()
      if (done) break
      
      const text = decoder.decode(value)
      const lines = text.split('\n').filter(line => line.trim())
      
      for (const line of lines) {
        try {
          const chunk = JSON.parse(line) as AIStreamChunk
          onChunk(chunk)
          
          if (chunk.type === 'error') {
            throw new Error(chunk.error)
          }
        } catch (e) {
          console.error('Failed to parse chunk:', e)
        }
      }
    }
  }
}

function getToken(): string {
  const session = localStorage.getItem('noval-auth-session')
  if (!session) return ''
  
  try {
    const CryptoJS = require('crypto-js')
    const decrypted = CryptoJS.AES.decrypt(session, config.encryptionKey).toString(CryptoJS.enc.Utf8)
    const data = JSON.parse(decrypted)
    return data.token || ''
  } catch {
    return ''
  }
}

export default aiApi