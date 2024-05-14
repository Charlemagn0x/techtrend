import axios, { AxiosError, AxiosResponse } from 'axios';

class ApiService {
  private baseUrl: string = process.env.REACT_APP_API_URL || '';

  private async makeRequest<T>(
    method: 'GET' | 'POST',
    url: string,
    data: object = {},
    params: object = {}
  ): Promise<T> {
    try {
      let response: AxiosResponse<T>;
      if (method === 'GET') {
        response = await axios.get<T>(`${this.baseUrl}/${url}`, { params });
      } else {
        response = await axios.post<T>(`${this.baseUrl}/${url}`, data);
      }
      return response.data;
    } catch (error) {
      // Improve error handling within makeRequest
      throw this.handleError(error);
    }
  }

  private handleError(error: unknown): Error {
    if (axios.isAxiosError(error)) {
      if (error.response) {
        console.error('Error data:', error.response.data);
        console.error('Error status:', error.response.status);
        console.error('Error headers:', error.response.headers);
        return new Error(`Error: ${error.response.status} ${error.message}`);
      } else if (error.request) {
        console.error('Error request:', error.request);
        return new Error('Error: The request was made but no response was received');
      }
    }
    console.error('Unexpected error:', error);
    return new Error('An unexpected error occurred');
  }

  async fetchTrendData(filters: Record<string, any> = {}): Promise<any> {
    try {
      return await this.makeRequest('GET', 'trends', {}, filters);
    } catch (error) {
      console.error('Failed to fetch trend data:', error);
      throw error; // Rethrow after logging to allow consumer to handle
    }
  }

  async sendUserQuery(query: string): Promise<any> {
    try {
      return await this.makeRequest('POST', 'user-query', { query });
    } catch (error) {
      console.error('Failed to send user query:', error);
      throw error; 
    }
  }

  async manageSubscription(domain: string, action: 'subscribe' | 'unsubscribe'): Promise<any> {
    try {
      return await this.makeRequest('POST', 'subscriptions', { domain, action });
    } catch (error) {
      console.error('Failed to manage subscription:', error);
      throw error; 
    }
  }
}

export const apiService = new ApiService();