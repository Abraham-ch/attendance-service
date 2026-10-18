import axios, { type InternalAxiosRequestConfig } from 'axios'

import { paths } from '@/paths'

import { demoAdapter, isDemo } from './demo'
import { getToken } from './utils'

const API_URL = import.meta.env.VITE_API_URL

export const api = axios.create({
  baseURL: API_URL,
  timeout: 5000,
  headers: {
    'Content-Type': 'application/json'
  }
})

api.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  const token = getToken()
  if (token) {
    config.headers.set('Authorization', `Bearer ${token}`)
  }
  if (isDemo()) config.adapter = demoAdapter
  return config
})

// A stored token the API rejects has expired: drop it and send the user back to login
api.interceptors.response.use(undefined, (error) => {
  if (error.response?.status === 401 && getToken()) {
    localStorage.removeItem('token')
    window.location.assign(paths.login)
  }
  return Promise.reject(error)
})
