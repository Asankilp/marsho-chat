
export interface AppConfig {
    marsho: {
      base_url?: string;
      api_key?: string;
      stream?: boolean;
      system_prompt?: string;
    };
    model: object;
  }
