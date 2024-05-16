import axios, { AxiosError, AxiosResponse } from 'axios';

class ApiService {
  private baseUrl: string = process.env.REACT_APP_API_URL || '';

  private HttpMethod = {
    GET: 'GET',
    POST: 'POST',
  } as const;

  type HttpMethod = typeof this.HttpMethod[keyof typeof this.HttpMethod];

  private async makeRequest<T>(
    method: HttpMethod,
    url: string,
    data: object = {},
    params: object = {}
  ): Promise<T> {
    try {
      let response: AxiosResponse<T>;
      const fullUrl = `${this.baseUrl}/${url}`;
      response = await axios({
        method: method,
        url: fullUrl,
        ...(method === this.HttpMethod.GET ? { params } : { data }),
      });

      return response.data;
    } catch (error) {
      throw this.handleError(error);
    }
  }

  private handleError(error: unknown): Error {
    if (axios.isAxiosError(error)) {
      const { response, request } = error;

      if (response) {
        console.error('Error data:', response.data);
        console.error('Error status:', response.status);
        console.error('Error headers:', response.headers);
        return new Error(`Error: ${response.status} ${error.message}`);
      } else if (request) {
        console.error('Error request:', request);
        return new Error('Error: The request was made but no response was received');
      }
    }
    console.error('Unexpected error:', error);
    return new Error('An unexpected error occurred');
  }

  async fetchTrendData(filters: Record<string, any> = {}): Promise<any> {
    try {
      return await this.makeRequest(this.HttpMethod.GET, 'trends', {}, filters);
    } catch (error) {
      console.error('Failed to fetch trend data:', error);
      throw error;
    }
  }

  async sendUserQuery(query: string): Promise<any> {
    try {
      return await this.makeRequest(this.HttpMethod.POST, 'user-query', { query });
    } catch (error) {
      console.error('Failed to send user query:', error);
      throw error;
    }
  }

  async manageSubscription(domain: string, action: 'subscribe' | 'unsubscribe'): Promise<any> {
    try {
      return await this.makeRequest(this.HttpMethod.POST, 'subscriptions', { domain, action });
    } catch (error) {
      console.error('Failed to manage subscription:', error);
      throw error;
    }
  }
}

export const apiService = new ApiService();