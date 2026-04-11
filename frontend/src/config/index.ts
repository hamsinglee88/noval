// 应用配置
// 注意：生产环境应使用环境变量

export const config = {
  // 加密密钥 - 应从环境变量获取
  encryptionKey: import.meta.env.VITE_ENCRYPTION_KEY || 'noval-dev-secret-key',
  
  // API 配置
  api: {
    baseURL: '/api',
    timeout: 60000,
  },
  
  // AI 配置
  ai: {
    maxTextLength: 50000,
    defaultTimeout: 60000,
  },
}

export default config