// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.

use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

fn get_error_class_name(e: &AnyError) -> &'static str {
  deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
  let module_loader = Rc::new(FsModuleLoader);
  let create_web_worker_cb = Arc::new(|_| {
    todo!("Web workers are not supported in the example");
  });
  let web_worker_event_cb = Arc::new(|_| {
    todo!("Web workers are not supported in the example");
  });

  let options = WorkerOptions {
    bootstrap: BootstrapOptions {
      args: vec![],
      cpu_count: 1,
      debug_flag: false,
      enable_testing_features: false,
      location: None,
      no_color: false,
      is_tty: false,
      runtime_version: "x".to_string(),
      ts_version: "x".to_string(),
      unstable: false,
      user_agent: "hello_runtime".to_string(),
      inspect: false,
    },
    extensions: vec![],
    unsafely_ignore_certificate_errors: None,
    root_cert_store: None,
    seed: None,
    source_map_getter: None,
    format_js_error_fn: None,
    web_worker_preload_module_cb: web_worker_event_cb.clone(),
    web_worker_pre_execute_module_cb: web_worker_event_cb,
    create_web_worker_cb,
    maybe_inspector_server: None,
    should_break_on_first_statement: false,
    module_loader,
    npm_resolver: None,
    get_error_class_fn: Some(&get_error_class_name),
    origin_storage_dir: None,
    blob_store: BlobStore::default(),
    broadcast_channel: InMemoryBroadcastChannel::default(),
    shared_array_buffer_store: None,
    compiled_wasm_module_store: None,
    stdio: Default::default(),
  };

  let js_path =
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/hello_runtime.js");
  let main_module = deno_core::resolve_path(&js_path.to_string_lossy())?;
  let permissions = Permissions::allow_all();

  let mut worker = MainWorker::bootstrap_from_options(
    main_module.clone(),
    permissions,
    options,
  );
//   worker.execute_main_module(&main_module).await?;
  worker.execute_script("zzf", r#"
  const urlJoin = function(...args) {
    let input;
    if (typeof args[0] === 'object') {
        input = args[0];
    } else {
        input = [].slice.call(args);
    }
    return normalize(input);
};
const normalize = (strArray)=>{
    const resultArray = [];
    if (strArray.length === 0) {
        return '';
    }
    if (typeof strArray[0] !== 'string') {
        throw new TypeError('Url must be a string. Received ' + strArray[0]);
    }
    if (strArray[0].match(/^[^/:]+:\/*$/) && strArray.length > 1) {
        const first = strArray.shift();
        strArray[0] = first + strArray[0];
    }
    if (strArray[0].match(/^file:\/\/\//)) {
        strArray[0] = strArray[0].replace(/^([^/:]+):\/*/, '$1:///');
    } else {
        strArray[0] = strArray[0].replace(/^([^/:]+):\/*/, '$1://');
    }
    for(let i = 0; i < strArray.length; i++){
        let component = strArray[i];
        if (typeof component !== 'string') {
            throw new TypeError('Url must be a string. Received ' + component);
        }
        if (component === '') {
            continue;
        }
        if (i > 0) {
            component = component.replace(/^[\/]+/, '');
        }
        if (i < strArray.length - 1) {
            component = component.replace(/[\/]+$/, '');
        } else {
            component = component.replace(/[\/]+$/, '/');
        }
        resultArray.push(component);
    }
    let str = resultArray.join('/');
    str = str.replace(/\/(\?|&|#[^!])/g, '$1');
    let parts = str.split('?');
    str = parts.shift() + (parts.length > 0 ? '?' : '') + parts.join('&');
    return str;
};
const methods = [
    'get',
    'post',
    'put',
    'delete',
    'options',
    'head',
    'connect',
    'trace',
    'patch', 
];
const addInterceptor = ()=>{
    const interceptor = {
        list: [],
        use: function(fulfilled, rejected) {
            const id = this.list.length;
            this.list.push({
                fulfilled,
                rejected
            });
            return id;
        },
        eject: function(index) {
            if (this.list[index]) {
                this.list[index] = null;
            }
        }
    };
    return interceptor;
};
function axiod(url, config) {
    if (typeof url === 'string') {
        return axiod.request(Object.assign({}, axiod.defaults, {
            url
        }, config));
    }
    return axiod.request(Object.assign({}, axiod.defaults, url));
}
axiod.defaults = {
    url: '/',
    method: 'get',
    timeout: 0,
    withCredentials: false,
    validateStatus: (status)=>{
        return status >= 200 && status < 300;
    }
};
axiod.create = (config)=>{
    const instance = axiod.bind({});
    instance.defaults = Object.assign({}, axiod.defaults, config);
    instance._request = request;
    instance.request = (options)=>{
        return instance._request(Object.assign({}, instance.defaults, options));
    };
    instance.get = (url, config)=>{
        return instance.request(Object.assign({}, {
            url
        }, config, {
            method: 'get'
        }));
    };
    instance.post = (url, data, config)=>{
        return instance.request(Object.assign({}, {
            url
        }, config, {
            method: 'post',
            data
        }));
    };
    instance.put = (url, data, config)=>{
        return instance.request(Object.assign({}, {
            url
        }, config, {
            method: 'put',
            data
        }));
    };
    instance.delete = (url, data, config)=>{
        return instance.request(Object.assign({}, {
            url
        }, config, {
            method: 'delete',
            data
        }));
    };
    instance.options = (url, data, config)=>{
        return instance.request(Object.assign({}, {
            url
        }, config, {
            method: 'options',
            data
        }));
    };
    instance.head = (url, data, config)=>{
        return instance.request(Object.assign({}, {
            url
        }, config, {
            method: 'head',
            data
        }));
    };
    instance.connect = (url, data, config)=>{
        return instance.request(Object.assign({}, {
            url
        }, config, {
            method: 'connect',
            data
        }));
    };
    instance.trace = (url, data, config)=>{
        return instance.request(Object.assign({}, {
            url
        }, config, {
            method: 'trace',
            data
        }));
    };
    instance.patch = (url, data, config)=>{
        return instance.request(Object.assign({}, {
            url
        }, config, {
            method: 'patch',
            data
        }));
    };
    instance.interceptors = {
        request: addInterceptor(),
        response: addInterceptor()
    };
    instance.interceptors.request.list = [];
    instance.interceptors.response.list = [];
    return instance;
};
async function request(config) {
    if (this.interceptors.request.list.length > 0) {
        for (const interceptor of this.interceptors.request.list){
            if (interceptor) {
                const { fulfilled  } = interceptor;
                if (fulfilled && config) {
                    config = await fulfilled(config);
                }
            }
        }
    }
    let { url ='/' , baseURL , method , headers , params ={} , data , timeout , withCredentials , auth , validateStatus , paramsSerializer , transformRequest , transformResponse , redirect , responseType ='json' ,  } = config;
    if (baseURL) {
        url = urlJoin(baseURL, url);
    }
    if (method) {
        if (methods.indexOf(method.toLowerCase().trim()) === -1) {
            throw new Error(`Method ${method} is not supported`);
        } else {
            method = method.toLowerCase().trim();
        }
    } else {
        method = 'get';
    }
    let _params = '';
    if (params) {
        if (paramsSerializer) {
            _params = paramsSerializer(params);
        } else {
            _params = Object.keys(params).map((key)=>{
                return encodeURIComponent(key) + '=' + encodeURIComponent(params[key]);
            }).join('&');
        }
    }
    if (withCredentials) {
        if (auth?.username && auth?.password) {
            if (!headers) {
                headers = {};
            }
            headers['Authorization'] = 'Basic ' + btoa(unescape(encodeURIComponent(`${auth.username}:${auth.password}`)));
        }
    }
    const fetchRequestObject = {};
    if (method !== 'get') {
        fetchRequestObject.method = method.toUpperCase();
    }
    if (_params) {
        url = urlJoin(url, `?${_params}`);
    }
    if (data && method !== 'get') {
        if (transformRequest && Array.isArray(transformRequest) && transformRequest.length > 0) {
            for(var i = 0; i < (transformRequest || []).length; i++){
                if (transformRequest && transformRequest[i]) {
                    data = transformRequest[i](data, headers);
                }
            }
        }
        if (typeof data === 'string' || data instanceof FormData || data instanceof URLSearchParams) {
            fetchRequestObject.body = data;
        } else {
            try {
                fetchRequestObject.body = JSON.stringify(data);
                if (!headers) {
                    headers = {};
                }
                headers['Accept'] = 'application/json';
                headers['Content-Type'] = 'application/json';
            } catch (ex) {}
        }
    }
    if (headers) {
        const _headers = new Headers();
        Object.keys(headers).forEach((header)=>{
            if (headers && headers[header]) {
                _headers.set(header, headers[header]);
            }
        });
        fetchRequestObject.headers = _headers;
    }
    const controller = new AbortController();
    fetchRequestObject.signal = controller.signal;
    let timeoutCounter = 0;
    if ((timeout || 0) > 0) {
        timeoutCounter = setTimeout(()=>{
            timeoutCounter = 0;
            controller.abort();
        }, timeout);
    }
    if (redirect) {
        fetchRequestObject.redirect = redirect;
    }
    return fetch(url, fetchRequestObject).then(async (x)=>{
        if (timeoutCounter) {
            clearTimeout(timeoutCounter);
        }
        const _status = x.status;
        const _statusText = x.statusText;
        let _data = null;
        try {
            const response = x.clone();
            if (responseType === 'json') {
                _data = await response.json();
            } else if (responseType === 'text') {
                _data = await response.text();
            } else if (responseType === 'arraybuffer') {
                _data = await response.arrayBuffer();
            } else if (responseType === 'blob') {
                _data = await response.blob();
            } else if (responseType === 'stream') {
                _data = (await response.blob()).stream();
            } else {
                _data = await response.text();
            }
        } catch (ex) {
            _data = await x.clone().text();
        }
        if (transformResponse) {
            if (transformResponse && Array.isArray(transformResponse) && transformResponse.length > 0) {
                for(var i = 0; i < (transformResponse || []).length; i++){
                    if (transformResponse && transformResponse[i]) {
                        _data = transformResponse[i](_data);
                    }
                }
            }
        }
        const _headers = x.headers;
        const _config = {
            url,
            baseURL,
            method,
            headers,
            params,
            data,
            timeout,
            withCredentials,
            auth,
            paramsSerializer,
            redirect,
            responseType
        };
        let isValidStatus = true;
        if (validateStatus) {
            isValidStatus = validateStatus(_status);
        } else {
            isValidStatus = _status >= 200 && _status <= 303;
        }
        let response1 = null;
        let error = null;
        if (isValidStatus) {
            response1 = {
                status: _status,
                statusText: _statusText,
                data: _data,
                headers: _headers,
                config: _config
            };
        } else {
            error = {
                response: {
                    status: _status,
                    statusText: _statusText,
                    data: _data,
                    headers: _headers
                },
                config: _config
            };
        }
        if (this.interceptors.response.list.length > 0) {
            for (const interceptor of this.interceptors.response.list){
                if (interceptor) {
                    const { fulfilled , rejected  } = interceptor;
                    if (fulfilled && response1) {
                        response1 = await fulfilled(response1);
                    }
                    if (rejected && error) {
                        error = await rejected(error);
                    }
                }
            }
        }
        if (error) {
            return Promise.reject(error);
        }
        return Promise.resolve(response1);
    });
}
axiod._request = request;
axiod.request = request;
axiod.get = (url, config)=>{
    return axiod.request(Object.assign({}, {
        url
    }, config, {
        method: 'get'
    }));
};
axiod.post = (url, data, config)=>{
    return axiod.request(Object.assign({}, {
        url
    }, config, {
        method: 'post',
        data
    }));
};
axiod.put = (url, data, config)=>{
    return axiod.request(Object.assign({}, {
        url
    }, config, {
        method: 'put',
        data
    }));
};
axiod.delete = (url, data, config)=>{
    return axiod.request(Object.assign({}, {
        url
    }, config, {
        method: 'delete',
        data
    }));
};
axiod.options = (url, data, config)=>{
    return axiod.request(Object.assign({}, {
        url
    }, config, {
        method: 'options',
        data
    }));
};
axiod.head = (url, data, config)=>{
    return axiod.request(Object.assign({}, {
        url
    }, config, {
        method: 'head',
        data
    }));
};
axiod.connect = (url, data, config)=>{
    return axiod.request(Object.assign({}, {
        url
    }, config, {
        method: 'connect',
        data
    }));
};
axiod.trace = (url, data, config)=>{
    return axiod.request(Object.assign({}, {
        url
    }, config, {
        method: 'trace',
        data
    }));
};
axiod.patch = (url, data, config)=>{
    return axiod.request(Object.assign({}, {
        url
    }, config, {
        method: 'patch',
        data
    }));
};
axiod.interceptors = {
    request: addInterceptor(),
    response: addInterceptor()
};
async function hello() {
    const { data  } = await axiod("https://postman-echo.com/delay/2");
    console.log(data);
}
hello();
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbImh0dHBzOi8vZGVuby5sYW5kL3gvdXJsX2pvaW5AMS4wLjAvbW9kLnRzIiwiaHR0cHM6Ly9kZW5vLmxhbmQveC9heGlvZEAwLjI2LjIvaGVscGVycy50cyIsImh0dHBzOi8vZGVuby5sYW5kL3gvYXhpb2RAMC4yNi4yL21vZC50cyIsImZpbGU6Ly8vZGF0YTAwL2hvbWUvemh1emhlbmZlbmcuY29kZS9yZXBvcy9naXRodWIvZGVub19mYWFzL2hlbGxvLnRzIl0sInNvdXJjZXNDb250ZW50IjpbImV4cG9ydCBjb25zdCB1cmxKb2luID0gZnVuY3Rpb24gKC4uLmFyZ3M6IGFueVtdKSB7XG4gIGxldCBpbnB1dDtcblxuICBpZiAodHlwZW9mIGFyZ3NbMF0gPT09ICdvYmplY3QnKSB7XG4gICAgaW5wdXQgPSBhcmdzWzBdO1xuICB9IGVsc2Uge1xuICAgIGlucHV0ID0gW10uc2xpY2UuY2FsbChhcmdzKTtcbiAgfVxuXG4gIHJldHVybiBub3JtYWxpemUoaW5wdXQpO1xufTtcblxuY29uc3Qgbm9ybWFsaXplID0gKHN0ckFycmF5OiBBcnJheTxzdHJpbmc+KSA9PiB7XG4gIGNvbnN0IHJlc3VsdEFycmF5ID0gW107XG4gIGlmIChzdHJBcnJheS5sZW5ndGggPT09IDApIHtcbiAgICByZXR1cm4gJyc7XG4gIH1cblxuICBpZiAodHlwZW9mIHN0ckFycmF5WzBdICE9PSAnc3RyaW5nJykge1xuICAgIHRocm93IG5ldyBUeXBlRXJyb3IoJ1VybCBtdXN0IGJlIGEgc3RyaW5nLiBSZWNlaXZlZCAnICsgc3RyQXJyYXlbMF0pO1xuICB9XG5cbiAgLy8gSWYgdGhlIGZpcnN0IHBhcnQgaXMgYSBwbGFpbiBwcm90b2NvbCwgd2UgY29tYmluZSBpdCB3aXRoIHRoZSBuZXh0IHBhcnQuXG4gIGlmIChzdHJBcnJheVswXS5tYXRjaCgvXlteLzpdKzpcXC8qJC8pICYmIHN0ckFycmF5Lmxlbmd0aCA+IDEpIHtcbiAgICBjb25zdCBmaXJzdCA9IHN0ckFycmF5LnNoaWZ0KCk7XG4gICAgc3RyQXJyYXlbMF0gPSBmaXJzdCArIHN0ckFycmF5WzBdO1xuICB9XG5cbiAgLy8gVGhlcmUgbXVzdCBiZSB0d28gb3IgdGhyZWUgc2xhc2hlcyBpbiB0aGUgZmlsZSBwcm90b2NvbCwgdHdvIHNsYXNoZXMgaW4gYW55dGhpbmcgZWxzZS5cbiAgaWYgKHN0ckFycmF5WzBdLm1hdGNoKC9eZmlsZTpcXC9cXC9cXC8vKSkge1xuICAgIHN0ckFycmF5WzBdID0gc3RyQXJyYXlbMF0ucmVwbGFjZSgvXihbXi86XSspOlxcLyovLCAnJDE6Ly8vJyk7XG4gIH0gZWxzZSB7XG4gICAgc3RyQXJyYXlbMF0gPSBzdHJBcnJheVswXS5yZXBsYWNlKC9eKFteLzpdKyk6XFwvKi8sICckMTovLycpO1xuICB9XG5cbiAgZm9yIChsZXQgaSA9IDA7IGkgPCBzdHJBcnJheS5sZW5ndGg7IGkrKykge1xuICAgIGxldCBjb21wb25lbnQgPSBzdHJBcnJheVtpXTtcblxuICAgIGlmICh0eXBlb2YgY29tcG9uZW50ICE9PSAnc3RyaW5nJykge1xuICAgICAgdGhyb3cgbmV3IFR5cGVFcnJvcignVXJsIG11c3QgYmUgYSBzdHJpbmcuIFJlY2VpdmVkICcgKyBjb21wb25lbnQpO1xuICAgIH1cblxuICAgIGlmIChjb21wb25lbnQgPT09ICcnKSB7XG4gICAgICBjb250aW51ZTtcbiAgICB9XG5cbiAgICBpZiAoaSA+IDApIHtcbiAgICAgIC8vIFJlbW92aW5nIHRoZSBzdGFydGluZyBzbGFzaGVzIGZvciBlYWNoIGNvbXBvbmVudCBidXQgdGhlIGZpcnN0LlxuICAgICAgY29tcG9uZW50ID0gY29tcG9uZW50LnJlcGxhY2UoL15bXFwvXSsvLCAnJyk7XG4gICAgfVxuICAgIGlmIChpIDwgc3RyQXJyYXkubGVuZ3RoIC0gMSkge1xuICAgICAgLy8gUmVtb3ZpbmcgdGhlIGVuZGluZyBzbGFzaGVzIGZvciBlYWNoIGNvbXBvbmVudCBidXQgdGhlIGxhc3QuXG4gICAgICBjb21wb25lbnQgPSBjb21wb25lbnQucmVwbGFjZSgvW1xcL10rJC8sICcnKTtcbiAgICB9IGVsc2Uge1xuICAgICAgLy8gRm9yIHRoZSBsYXN0IGNvbXBvbmVudCB3ZSB3aWxsIGNvbWJpbmUgbXVsdGlwbGUgc2xhc2hlcyB0byBhIHNpbmdsZSBvbmUuXG4gICAgICBjb21wb25lbnQgPSBjb21wb25lbnQucmVwbGFjZSgvW1xcL10rJC8sICcvJyk7XG4gICAgfVxuXG4gICAgcmVzdWx0QXJyYXkucHVzaChjb21wb25lbnQpO1xuICB9XG5cbiAgbGV0IHN0ciA9IHJlc3VsdEFycmF5LmpvaW4oJy8nKTtcbiAgLy8gRWFjaCBpbnB1dCBjb21wb25lbnQgaXMgbm93IHNlcGFyYXRlZCBieSBhIHNpbmdsZSBzbGFzaCBleGNlcHQgdGhlIHBvc3NpYmxlIGZpcnN0IHBsYWluIHByb3RvY29sIHBhcnQuXG5cbiAgLy8gcmVtb3ZlIHRyYWlsaW5nIHNsYXNoIGJlZm9yZSBwYXJhbWV0ZXJzIG9yIGhhc2hcbiAgc3RyID0gc3RyLnJlcGxhY2UoL1xcLyhcXD98JnwjW14hXSkvZywgJyQxJyk7XG5cbiAgLy8gcmVwbGFjZSA/IGluIHBhcmFtZXRlcnMgd2l0aCAmXG4gIGxldCBwYXJ0cyA9IHN0ci5zcGxpdCgnPycpO1xuICBzdHIgPSBwYXJ0cy5zaGlmdCgpICsgKHBhcnRzLmxlbmd0aCA+IDAgPyAnPycgOiAnJykgKyBwYXJ0cy5qb2luKCcmJyk7XG5cbiAgcmV0dXJuIHN0cjtcbn07XG4iLCJpbXBvcnQgeyBJQXhpb2RJbnRlcmNlcHRvciB9IGZyb20gJy4vaW50ZXJmYWNlcy50cyc7XG5cbmV4cG9ydCBjb25zdCBtZXRob2RzID0gW1xuICAnZ2V0JyxcbiAgJ3Bvc3QnLFxuICAncHV0JyxcbiAgJ2RlbGV0ZScsXG4gICdvcHRpb25zJyxcbiAgJ2hlYWQnLFxuICAnY29ubmVjdCcsXG4gICd0cmFjZScsXG4gICdwYXRjaCcsXG5dO1xuXG5leHBvcnQgY29uc3QgYWRkSW50ZXJjZXB0b3IgPSA8RnVsbGZpbGwgPSB1bmtub3duLCBSZWplY3RlZCA9IHVua25vd24+KCkgPT4ge1xuICBjb25zdCBpbnRlcmNlcHRvcjogSUF4aW9kSW50ZXJjZXB0b3I8RnVsbGZpbGwsIFJlamVjdGVkPiA9IHtcbiAgICBsaXN0OiBbXSxcbiAgICB1c2U6IGZ1bmN0aW9uIChmdWxmaWxsZWQsIHJlamVjdGVkKSB7XG4gICAgICBjb25zdCBpZCA9IHRoaXMubGlzdC5sZW5ndGg7XG5cbiAgICAgIHRoaXMubGlzdC5wdXNoKHtcbiAgICAgICAgZnVsZmlsbGVkLFxuICAgICAgICByZWplY3RlZCxcbiAgICAgIH0pO1xuXG4gICAgICByZXR1cm4gaWQ7XG4gICAgfSxcbiAgICBlamVjdDogZnVuY3Rpb24gKGluZGV4KSB7XG4gICAgICBpZiAodGhpcy5saXN0W2luZGV4XSkge1xuICAgICAgICB0aGlzLmxpc3RbaW5kZXhdID0gbnVsbDtcbiAgICAgIH1cbiAgICB9LFxuICB9O1xuXG4gIHJldHVybiBpbnRlcmNlcHRvcjtcbn07XG4iLCJpbXBvcnQgeyB1cmxKb2luIH0gZnJvbSAnLi9tb2RzL3VybC1qb2luLnRzJztcblxuaW1wb3J0IHR5cGUge1xuICBEYXRhLFxuICBJQXhpb2RFcnJvcixcbiAgSUF4aW9kUmVxdWVzdEVycm9ySW50ZXJjZXB0b3IsXG4gIElBeGlvZFJlcXVlc3RJbnRlcmNlcHRvcixcbiAgSUF4aW9kUmVzcG9uc2UsXG4gIElBeGlvZFJlc3BvbnNlRXJyb3JJbnRlcmNlcHRvcixcbiAgSUF4aW9kUmVzcG9uc2VJbnRlcmNlcHRvcixcbiAgSUNvbmZpZyxcbiAgSVJlcXVlc3QsXG59IGZyb20gJy4vaW50ZXJmYWNlcy50cyc7XG5pbXBvcnQgeyBhZGRJbnRlcmNlcHRvciwgbWV0aG9kcyB9IGZyb20gJy4vaGVscGVycy50cyc7XG5cbmZ1bmN0aW9uIGF4aW9kPFQgPSBhbnk+KFxuICB1cmw6IHN0cmluZyB8IElSZXF1ZXN0LFxuICBjb25maWc/OiBJUmVxdWVzdCxcbik6IFByb21pc2U8SUF4aW9kUmVzcG9uc2U8VD4+IHtcbiAgaWYgKHR5cGVvZiB1cmwgPT09ICdzdHJpbmcnKSB7XG4gICAgcmV0dXJuIGF4aW9kLnJlcXVlc3QoT2JqZWN0LmFzc2lnbih7fSwgYXhpb2QuZGVmYXVsdHMsIHsgdXJsIH0sIGNvbmZpZykpO1xuICB9XG4gIHJldHVybiBheGlvZC5yZXF1ZXN0KE9iamVjdC5hc3NpZ24oe30sIGF4aW9kLmRlZmF1bHRzLCB1cmwpKTtcbn1cblxuYXhpb2QuZGVmYXVsdHMgPSB7XG4gIHVybDogJy8nLFxuICBtZXRob2Q6ICdnZXQnLFxuICB0aW1lb3V0OiAwLFxuICB3aXRoQ3JlZGVudGlhbHM6IGZhbHNlLFxuICB2YWxpZGF0ZVN0YXR1czogKHN0YXR1czogbnVtYmVyKSA9PiB7XG4gICAgcmV0dXJuIHN0YXR1cyA+PSAyMDAgJiYgc3RhdHVzIDwgMzAwO1xuICB9LFxufTtcblxuYXhpb2QuY3JlYXRlID0gKGNvbmZpZz86IElSZXF1ZXN0KSA9PiB7XG4gIGNvbnN0IGluc3RhbmNlID0gYXhpb2QuYmluZCh7fSk7XG4gIGluc3RhbmNlLmRlZmF1bHRzID0gT2JqZWN0LmFzc2lnbih7fSwgYXhpb2QuZGVmYXVsdHMsIGNvbmZpZyk7XG5cbiAgaW5zdGFuY2UuX3JlcXVlc3QgPSByZXF1ZXN0O1xuXG4gIGluc3RhbmNlLnJlcXVlc3QgPSAob3B0aW9uczogSVJlcXVlc3QpOiBQcm9taXNlPElBeGlvZFJlc3BvbnNlPiA9PiB7XG4gICAgcmV0dXJuIGluc3RhbmNlLl9yZXF1ZXN0KE9iamVjdC5hc3NpZ24oe30sIGluc3RhbmNlLmRlZmF1bHRzLCBvcHRpb25zKSk7XG4gIH07XG4gIGluc3RhbmNlLmdldCA9ICh1cmw6IHN0cmluZywgY29uZmlnPzogSUNvbmZpZykgPT4ge1xuICAgIHJldHVybiBpbnN0YW5jZS5yZXF1ZXN0KFxuICAgICAgT2JqZWN0LmFzc2lnbih7fSwgeyB1cmwgfSwgY29uZmlnLCB7IG1ldGhvZDogJ2dldCcgfSksXG4gICAgKTtcbiAgfTtcbiAgaW5zdGFuY2UucG9zdCA9ICh1cmw6IHN0cmluZywgZGF0YT86IERhdGEsIGNvbmZpZz86IElDb25maWcpID0+IHtcbiAgICByZXR1cm4gaW5zdGFuY2UucmVxdWVzdChcbiAgICAgIE9iamVjdC5hc3NpZ24oe30sIHsgdXJsIH0sIGNvbmZpZywgeyBtZXRob2Q6ICdwb3N0JywgZGF0YSB9KSxcbiAgICApO1xuICB9O1xuICBpbnN0YW5jZS5wdXQgPSAodXJsOiBzdHJpbmcsIGRhdGE/OiBEYXRhLCBjb25maWc/OiBJQ29uZmlnKSA9PiB7XG4gICAgcmV0dXJuIGluc3RhbmNlLnJlcXVlc3QoXG4gICAgICBPYmplY3QuYXNzaWduKHt9LCB7IHVybCB9LCBjb25maWcsIHsgbWV0aG9kOiAncHV0JywgZGF0YSB9KSxcbiAgICApO1xuICB9O1xuICBpbnN0YW5jZS5kZWxldGUgPSAodXJsOiBzdHJpbmcsIGRhdGE/OiBEYXRhLCBjb25maWc/OiBJQ29uZmlnKSA9PiB7XG4gICAgcmV0dXJuIGluc3RhbmNlLnJlcXVlc3QoXG4gICAgICBPYmplY3QuYXNzaWduKHt9LCB7IHVybCB9LCBjb25maWcsIHsgbWV0aG9kOiAnZGVsZXRlJywgZGF0YSB9KSxcbiAgICApO1xuICB9O1xuICBpbnN0YW5jZS5vcHRpb25zID0gKHVybDogc3RyaW5nLCBkYXRhPzogRGF0YSwgY29uZmlnPzogSUNvbmZpZykgPT4ge1xuICAgIHJldHVybiBpbnN0YW5jZS5yZXF1ZXN0KFxuICAgICAgT2JqZWN0LmFzc2lnbih7fSwgeyB1cmwgfSwgY29uZmlnLCB7IG1ldGhvZDogJ29wdGlvbnMnLCBkYXRhIH0pLFxuICAgICk7XG4gIH07XG4gIGluc3RhbmNlLmhlYWQgPSAodXJsOiBzdHJpbmcsIGRhdGE/OiBEYXRhLCBjb25maWc/OiBJQ29uZmlnKSA9PiB7XG4gICAgcmV0dXJuIGluc3RhbmNlLnJlcXVlc3QoXG4gICAgICBPYmplY3QuYXNzaWduKHt9LCB7IHVybCB9LCBjb25maWcsIHsgbWV0aG9kOiAnaGVhZCcsIGRhdGEgfSksXG4gICAgKTtcbiAgfTtcbiAgaW5zdGFuY2UuY29ubmVjdCA9ICh1cmw6IHN0cmluZywgZGF0YT86IERhdGEsIGNvbmZpZz86IElDb25maWcpID0+IHtcbiAgICByZXR1cm4gaW5zdGFuY2UucmVxdWVzdChcbiAgICAgIE9iamVjdC5hc3NpZ24oe30sIHsgdXJsIH0sIGNvbmZpZywgeyBtZXRob2Q6ICdjb25uZWN0JywgZGF0YSB9KSxcbiAgICApO1xuICB9O1xuICBpbnN0YW5jZS50cmFjZSA9ICh1cmw6IHN0cmluZywgZGF0YT86IERhdGEsIGNvbmZpZz86IElDb25maWcpID0+IHtcbiAgICByZXR1cm4gaW5zdGFuY2UucmVxdWVzdChcbiAgICAgIE9iamVjdC5hc3NpZ24oe30sIHsgdXJsIH0sIGNvbmZpZywgeyBtZXRob2Q6ICd0cmFjZScsIGRhdGEgfSksXG4gICAgKTtcbiAgfTtcbiAgaW5zdGFuY2UucGF0Y2ggPSAodXJsOiBzdHJpbmcsIGRhdGE/OiBEYXRhLCBjb25maWc/OiBJQ29uZmlnKSA9PiB7XG4gICAgcmV0dXJuIGluc3RhbmNlLnJlcXVlc3QoXG4gICAgICBPYmplY3QuYXNzaWduKHt9LCB7IHVybCB9LCBjb25maWcsIHsgbWV0aG9kOiAncGF0Y2gnLCBkYXRhIH0pLFxuICAgICk7XG4gIH07XG5cbiAgaW5zdGFuY2UuaW50ZXJjZXB0b3JzID0ge1xuICAgIHJlcXVlc3Q6IGFkZEludGVyY2VwdG9yPElBeGlvZFJlcXVlc3RJbnRlcmNlcHRvciwgSUF4aW9kUmVxdWVzdEVycm9ySW50ZXJjZXB0b3I+KCksXG4gICAgcmVzcG9uc2U6IGFkZEludGVyY2VwdG9yPElBeGlvZFJlc3BvbnNlSW50ZXJjZXB0b3IsIElBeGlvZFJlc3BvbnNlRXJyb3JJbnRlcmNlcHRvcj4oKSxcbiAgfTtcblxuICBpbnN0YW5jZS5pbnRlcmNlcHRvcnMucmVxdWVzdC5saXN0ID0gW107XG4gIGluc3RhbmNlLmludGVyY2VwdG9ycy5yZXNwb25zZS5saXN0ID0gW107XG5cbiAgcmV0dXJuIGluc3RhbmNlO1xufTtcblxuYXN5bmMgZnVuY3Rpb24gcmVxdWVzdDxUID0gYW55Pih0aGlzOiB0eXBlb2YgYXhpb2QsIGNvbmZpZzogSVJlcXVlc3QpOiBQcm9taXNlPElBeGlvZFJlc3BvbnNlPFQ+PiB7XG4gIGlmICh0aGlzLmludGVyY2VwdG9ycy5yZXF1ZXN0Lmxpc3QubGVuZ3RoID4gMCkge1xuICAgIGZvciAoY29uc3QgaW50ZXJjZXB0b3Igb2YgdGhpcy5pbnRlcmNlcHRvcnMucmVxdWVzdC5saXN0KSB7XG4gICAgICBpZiAoaW50ZXJjZXB0b3IpIHtcbiAgICAgICAgY29uc3QgeyBmdWxmaWxsZWQgfSA9IGludGVyY2VwdG9yO1xuICAgICAgICBpZiAoZnVsZmlsbGVkICYmIGNvbmZpZykge1xuICAgICAgICAgIGNvbmZpZyA9IGF3YWl0IGZ1bGZpbGxlZChjb25maWcpO1xuICAgICAgICB9XG4gICAgICB9XG4gICAgfVxuICB9XG5cbiAgbGV0IHtcbiAgICB1cmwgPSAnLycsXG4gICAgYmFzZVVSTCxcbiAgICBtZXRob2QsXG4gICAgaGVhZGVycyxcbiAgICBwYXJhbXMgPSB7fSxcbiAgICBkYXRhLFxuICAgIHRpbWVvdXQsXG4gICAgd2l0aENyZWRlbnRpYWxzLFxuICAgIGF1dGgsXG4gICAgdmFsaWRhdGVTdGF0dXMsXG4gICAgcGFyYW1zU2VyaWFsaXplcixcbiAgICB0cmFuc2Zvcm1SZXF1ZXN0LFxuICAgIHRyYW5zZm9ybVJlc3BvbnNlLFxuICAgIHJlZGlyZWN0LFxuICAgIHJlc3BvbnNlVHlwZSA9ICdqc29uJyxcbiAgfSA9IGNvbmZpZztcblxuICAvLyBVcmwgYW5kIEJhc2UgdXJsXG4gIGlmIChiYXNlVVJMKSB7XG4gICAgdXJsID0gdXJsSm9pbihiYXNlVVJMLCB1cmwpO1xuICB9XG5cbiAgLy8gTWV0aG9kXG4gIGlmIChtZXRob2QpIHtcbiAgICBpZiAobWV0aG9kcy5pbmRleE9mKG1ldGhvZC50b0xvd2VyQ2FzZSgpLnRyaW0oKSkgPT09IC0xKSB7XG4gICAgICB0aHJvdyBuZXcgRXJyb3IoYE1ldGhvZCAke21ldGhvZH0gaXMgbm90IHN1cHBvcnRlZGApO1xuICAgIH0gZWxzZSB7XG4gICAgICBtZXRob2QgPSBtZXRob2QudG9Mb3dlckNhc2UoKS50cmltKCk7XG4gICAgfVxuICB9IGVsc2Uge1xuICAgIG1ldGhvZCA9ICdnZXQnO1xuICB9XG5cbiAgLy8gUGFyYW1zXG4gIGxldCBfcGFyYW1zID0gJyc7XG4gIGlmIChwYXJhbXMpIHtcbiAgICBpZiAocGFyYW1zU2VyaWFsaXplcikge1xuICAgICAgX3BhcmFtcyA9IHBhcmFtc1NlcmlhbGl6ZXIocGFyYW1zKTtcbiAgICB9IGVsc2Uge1xuICAgICAgX3BhcmFtcyA9IE9iamVjdC5rZXlzKHBhcmFtcylcbiAgICAgICAgLm1hcCgoa2V5KSA9PiB7XG4gICAgICAgICAgcmV0dXJuIChcbiAgICAgICAgICAgIGVuY29kZVVSSUNvbXBvbmVudChrZXkpICsgJz0nICsgZW5jb2RlVVJJQ29tcG9uZW50KHBhcmFtc1trZXldKVxuICAgICAgICAgICk7XG4gICAgICAgIH0pXG4gICAgICAgIC5qb2luKCcmJyk7XG4gICAgfVxuICB9XG5cbiAgLy8gQWRkIGNyZWRlbnRpYWxzIHRvIGhlYWRlclxuICBpZiAod2l0aENyZWRlbnRpYWxzKSB7XG4gICAgaWYgKGF1dGg/LnVzZXJuYW1lICYmIGF1dGg/LnBhc3N3b3JkKSB7XG4gICAgICBpZiAoIWhlYWRlcnMpIHtcbiAgICAgICAgaGVhZGVycyA9IHt9O1xuICAgICAgfVxuXG4gICAgICBoZWFkZXJzWydBdXRob3JpemF0aW9uJ10gPSAnQmFzaWMgJyArXG4gICAgICAgIGJ0b2EodW5lc2NhcGUoZW5jb2RlVVJJQ29tcG9uZW50KGAke2F1dGgudXNlcm5hbWV9OiR7YXV0aC5wYXNzd29yZH1gKSkpO1xuICAgIH1cbiAgfVxuXG4gIC8vIENyZWF0ZSBmZXRjaCBSZXF1ZXN0IENvbmZpZ1xuICBjb25zdCBmZXRjaFJlcXVlc3RPYmplY3Q6IFJlcXVlc3RJbml0ID0ge307XG5cbiAgLy8gQWRkIG1ldGhvZCB0byBSZXF1ZXN0IENvbmZpZ1xuICBpZiAobWV0aG9kICE9PSAnZ2V0Jykge1xuICAgIGZldGNoUmVxdWVzdE9iamVjdC5tZXRob2QgPSBtZXRob2QudG9VcHBlckNhc2UoKTtcbiAgfVxuXG4gIC8vIEFkZCBwYXJhbXMgdG8gUmVxdWVzdCBDb25maWcgVXJsXG4gIGlmIChfcGFyYW1zKSB7XG4gICAgdXJsID0gdXJsSm9pbih1cmwsIGA/JHtfcGFyYW1zfWApO1xuICB9XG5cbiAgLy8gQWRkIGJvZHkgdG8gUmVxdWVzdCBDb25maWdcbiAgaWYgKGRhdGEgJiYgbWV0aG9kICE9PSAnZ2V0Jykge1xuICAgIC8vIHRyYW5zZm9ybVJlcXVlc3RcbiAgICBpZiAoXG4gICAgICB0cmFuc2Zvcm1SZXF1ZXN0ICYmXG4gICAgICBBcnJheS5pc0FycmF5KHRyYW5zZm9ybVJlcXVlc3QpICYmXG4gICAgICB0cmFuc2Zvcm1SZXF1ZXN0Lmxlbmd0aCA+IDBcbiAgICApIHtcbiAgICAgIGZvciAodmFyIGkgPSAwOyBpIDwgKHRyYW5zZm9ybVJlcXVlc3QgfHwgW10pLmxlbmd0aDsgaSsrKSB7XG4gICAgICAgIGlmICh0cmFuc2Zvcm1SZXF1ZXN0ICYmIHRyYW5zZm9ybVJlcXVlc3RbaV0pIHtcbiAgICAgICAgICBkYXRhID0gdHJhbnNmb3JtUmVxdWVzdFtpXShkYXRhLCBoZWFkZXJzKTtcbiAgICAgICAgfVxuICAgICAgfVxuICAgIH1cblxuICAgIGlmIChcbiAgICAgIHR5cGVvZiBkYXRhID09PSAnc3RyaW5nJyB8fFxuICAgICAgZGF0YSBpbnN0YW5jZW9mIEZvcm1EYXRhIHx8XG4gICAgICBkYXRhIGluc3RhbmNlb2YgVVJMU2VhcmNoUGFyYW1zXG4gICAgKSB7XG4gICAgICBmZXRjaFJlcXVlc3RPYmplY3QuYm9keSA9IGRhdGE7XG4gICAgfSBlbHNlIHtcbiAgICAgIHRyeSB7XG4gICAgICAgIGZldGNoUmVxdWVzdE9iamVjdC5ib2R5ID0gSlNPTi5zdHJpbmdpZnkoZGF0YSk7XG4gICAgICAgIGlmICghaGVhZGVycykge1xuICAgICAgICAgIGhlYWRlcnMgPSB7fTtcbiAgICAgICAgfVxuXG4gICAgICAgIGhlYWRlcnNbJ0FjY2VwdCddID0gJ2FwcGxpY2F0aW9uL2pzb24nO1xuICAgICAgICBoZWFkZXJzWydDb250ZW50LVR5cGUnXSA9ICdhcHBsaWNhdGlvbi9qc29uJztcbiAgICAgIH0gY2F0Y2ggKGV4KSB7fVxuICAgIH1cbiAgfVxuXG4gIC8vIEFkZCBoZWFkZXJzIHRvIFJlcXVlc3QgQ29uZmlnXG4gIGlmIChoZWFkZXJzKSB7XG4gICAgY29uc3QgX2hlYWRlcnM6IEhlYWRlcnMgPSBuZXcgSGVhZGVycygpO1xuICAgIE9iamVjdC5rZXlzKGhlYWRlcnMpLmZvckVhY2goKGhlYWRlcikgPT4ge1xuICAgICAgaWYgKGhlYWRlcnMgJiYgaGVhZGVyc1toZWFkZXJdKSB7XG4gICAgICAgIF9oZWFkZXJzLnNldChoZWFkZXIsIGhlYWRlcnNbaGVhZGVyXSk7XG4gICAgICB9XG4gICAgfSk7XG4gICAgZmV0Y2hSZXF1ZXN0T2JqZWN0LmhlYWRlcnMgPSBfaGVhZGVycztcbiAgfVxuXG4gIC8vIFRpbWVvdXRcbiAgY29uc3QgY29udHJvbGxlciA9IG5ldyBBYm9ydENvbnRyb2xsZXIoKTtcbiAgZmV0Y2hSZXF1ZXN0T2JqZWN0LnNpZ25hbCA9IGNvbnRyb2xsZXIuc2lnbmFsO1xuXG4gIGxldCB0aW1lb3V0Q291bnRlcjogbnVtYmVyID0gMDtcblxuICBpZiAoKHRpbWVvdXQgfHwgMCkgPiAwKSB7XG4gICAgdGltZW91dENvdW50ZXIgPSBzZXRUaW1lb3V0KCgpID0+IHtcbiAgICAgIHRpbWVvdXRDb3VudGVyID0gMDtcbiAgICAgIGNvbnRyb2xsZXIuYWJvcnQoKTtcbiAgICB9LCB0aW1lb3V0KTtcbiAgfVxuXG4gIGlmIChyZWRpcmVjdCkge1xuICAgIGZldGNoUmVxdWVzdE9iamVjdC5yZWRpcmVjdCA9IHJlZGlyZWN0O1xuICB9XG5cbiAgLy8gU3RhcnQgcmVxdWVzdFxuICByZXR1cm4gZmV0Y2godXJsLCBmZXRjaFJlcXVlc3RPYmplY3QpLnRoZW4oYXN5bmMgKHgpID0+IHtcbiAgICAvLyBDbGVhciB0aW1lb3V0XG4gICAgaWYgKHRpbWVvdXRDb3VudGVyKSB7XG4gICAgICBjbGVhclRpbWVvdXQodGltZW91dENvdW50ZXIpO1xuICAgIH1cblxuICAgIGNvbnN0IF9zdGF0dXM6IG51bWJlciA9IHguc3RhdHVzO1xuICAgIGNvbnN0IF9zdGF0dXNUZXh0OiBzdHJpbmcgPSB4LnN0YXR1c1RleHQ7XG5cbiAgICAvLyBEYXRhXG4gICAgbGV0IF9kYXRhOiBhbnkgPSBudWxsO1xuXG4gICAgLy8gVHJ5IHRvIGF1dG8gcGFyc2UgZGF0YVxuICAgIHRyeSB7XG4gICAgICBjb25zdCByZXNwb25zZSA9IHguY2xvbmUoKTtcblxuICAgICAgaWYgKHJlc3BvbnNlVHlwZSA9PT0gJ2pzb24nKSB7XG4gICAgICAgIF9kYXRhID0gYXdhaXQgcmVzcG9uc2UuanNvbigpO1xuICAgICAgfSBlbHNlIGlmIChyZXNwb25zZVR5cGUgPT09ICd0ZXh0Jykge1xuICAgICAgICBfZGF0YSA9IGF3YWl0IHJlc3BvbnNlLnRleHQoKTtcbiAgICAgIH0gZWxzZSBpZiAocmVzcG9uc2VUeXBlID09PSAnYXJyYXlidWZmZXInKSB7XG4gICAgICAgIF9kYXRhID0gYXdhaXQgcmVzcG9uc2UuYXJyYXlCdWZmZXIoKTtcbiAgICAgIH0gZWxzZSBpZiAocmVzcG9uc2VUeXBlID09PSAnYmxvYicpIHtcbiAgICAgICAgX2RhdGEgPSBhd2FpdCByZXNwb25zZS5ibG9iKCk7XG4gICAgICB9IGVsc2UgaWYgKHJlc3BvbnNlVHlwZSA9PT0gJ3N0cmVhbScpIHtcbiAgICAgICAgX2RhdGEgPSAoYXdhaXQgcmVzcG9uc2UuYmxvYigpKS5zdHJlYW0oKTtcbiAgICAgIH0gZWxzZSB7XG4gICAgICAgIF9kYXRhID0gYXdhaXQgcmVzcG9uc2UudGV4dCgpO1xuICAgICAgfVxuICAgIH0gY2F0Y2ggKGV4KSB7XG4gICAgICBfZGF0YSA9IGF3YWl0IHguY2xvbmUoKS50ZXh0KCk7XG4gICAgfVxuXG4gICAgLy8gdHJhbnNmb3JtUmVzcG9uc2VcbiAgICBpZiAodHJhbnNmb3JtUmVzcG9uc2UpIHtcbiAgICAgIGlmIChcbiAgICAgICAgdHJhbnNmb3JtUmVzcG9uc2UgJiZcbiAgICAgICAgQXJyYXkuaXNBcnJheSh0cmFuc2Zvcm1SZXNwb25zZSkgJiZcbiAgICAgICAgdHJhbnNmb3JtUmVzcG9uc2UubGVuZ3RoID4gMFxuICAgICAgKSB7XG4gICAgICAgIGZvciAodmFyIGkgPSAwOyBpIDwgKHRyYW5zZm9ybVJlc3BvbnNlIHx8IFtdKS5sZW5ndGg7IGkrKykge1xuICAgICAgICAgIGlmICh0cmFuc2Zvcm1SZXNwb25zZSAmJiB0cmFuc2Zvcm1SZXNwb25zZVtpXSkge1xuICAgICAgICAgICAgX2RhdGEgPSB0cmFuc2Zvcm1SZXNwb25zZVtpXShfZGF0YSk7XG4gICAgICAgICAgfVxuICAgICAgICB9XG4gICAgICB9XG4gICAgfVxuXG4gICAgY29uc3QgX2hlYWRlcnM6IEhlYWRlcnMgPSB4LmhlYWRlcnM7XG4gICAgY29uc3QgX2NvbmZpZzogSVJlcXVlc3QgPSB7XG4gICAgICB1cmwsXG4gICAgICBiYXNlVVJMLFxuICAgICAgbWV0aG9kLFxuICAgICAgaGVhZGVycyxcbiAgICAgIHBhcmFtcyxcbiAgICAgIGRhdGEsXG4gICAgICB0aW1lb3V0LFxuICAgICAgd2l0aENyZWRlbnRpYWxzLFxuICAgICAgYXV0aCxcbiAgICAgIHBhcmFtc1NlcmlhbGl6ZXIsXG4gICAgICByZWRpcmVjdCxcbiAgICAgIHJlc3BvbnNlVHlwZSxcbiAgICB9O1xuXG4gICAgLy8gVmFsaWRhdGUgdGhlIHN0YXR1cyBjb2RlXG4gICAgbGV0IGlzVmFsaWRTdGF0dXMgPSB0cnVlO1xuXG4gICAgaWYgKHZhbGlkYXRlU3RhdHVzKSB7XG4gICAgICBpc1ZhbGlkU3RhdHVzID0gdmFsaWRhdGVTdGF0dXMoX3N0YXR1cyk7XG4gICAgfSBlbHNlIHtcbiAgICAgIGlzVmFsaWRTdGF0dXMgPSBfc3RhdHVzID49IDIwMCAmJiBfc3RhdHVzIDw9IDMwMztcbiAgICB9XG5cbiAgICBsZXQgcmVzcG9uc2U6IElBeGlvZFJlc3BvbnNlPFQ+IHwgbnVsbCA9IG51bGw7XG4gICAgbGV0IGVycm9yOiBJQXhpb2RFcnJvcjxUPiB8IG51bGwgPSBudWxsO1xuXG4gICAgaWYgKGlzVmFsaWRTdGF0dXMpIHtcbiAgICAgIHJlc3BvbnNlID0ge1xuICAgICAgICBzdGF0dXM6IF9zdGF0dXMsXG4gICAgICAgIHN0YXR1c1RleHQ6IF9zdGF0dXNUZXh0LFxuICAgICAgICBkYXRhOiBfZGF0YSxcbiAgICAgICAgaGVhZGVyczogX2hlYWRlcnMsXG4gICAgICAgIGNvbmZpZzogX2NvbmZpZyxcbiAgICAgIH07XG4gICAgfSBlbHNlIHtcbiAgICAgIGVycm9yID0ge1xuICAgICAgICByZXNwb25zZToge1xuICAgICAgICAgIHN0YXR1czogX3N0YXR1cyxcbiAgICAgICAgICBzdGF0dXNUZXh0OiBfc3RhdHVzVGV4dCxcbiAgICAgICAgICBkYXRhOiBfZGF0YSxcbiAgICAgICAgICBoZWFkZXJzOiBfaGVhZGVycyxcbiAgICAgICAgfSxcbiAgICAgICAgY29uZmlnOiBfY29uZmlnLFxuICAgICAgfTtcbiAgICB9XG5cbiAgICBpZiAodGhpcy5pbnRlcmNlcHRvcnMucmVzcG9uc2UubGlzdC5sZW5ndGggPiAwKSB7XG4gICAgICBmb3IgKGNvbnN0IGludGVyY2VwdG9yIG9mIHRoaXMuaW50ZXJjZXB0b3JzLnJlc3BvbnNlLmxpc3QpIHtcbiAgICAgICAgaWYgKGludGVyY2VwdG9yKSB7XG4gICAgICAgICAgY29uc3QgeyBmdWxmaWxsZWQsIHJlamVjdGVkIH0gPSBpbnRlcmNlcHRvcjtcbiAgICAgICAgICBpZiAoZnVsZmlsbGVkICYmIHJlc3BvbnNlKSB7XG4gICAgICAgICAgICByZXNwb25zZSA9IChhd2FpdCBmdWxmaWxsZWQocmVzcG9uc2UpKSBhcyBJQXhpb2RSZXNwb25zZTxUPjtcbiAgICAgICAgICB9XG4gICAgICAgICAgaWYgKHJlamVjdGVkICYmIGVycm9yKSB7XG4gICAgICAgICAgICBlcnJvciA9IGF3YWl0IHJlamVjdGVkKGVycm9yKTtcbiAgICAgICAgICB9XG4gICAgICAgIH1cbiAgICAgIH1cbiAgICB9XG5cbiAgICBpZiAoZXJyb3IpIHtcbiAgICAgIHJldHVybiBQcm9taXNlLnJlamVjdChlcnJvciBhcyBJQXhpb2RFcnJvcjxUPik7XG4gICAgfVxuXG4gICAgcmV0dXJuIFByb21pc2UucmVzb2x2ZShyZXNwb25zZSBhcyBJQXhpb2RSZXNwb25zZTxUPik7XG4gIH0pO1xufVxuXG5heGlvZC5fcmVxdWVzdCA9IHJlcXVlc3Q7XG5heGlvZC5yZXF1ZXN0ID0gcmVxdWVzdDtcblxuYXhpb2QuZ2V0ID0gPFQgPSBhbnk+KHVybDogc3RyaW5nLCBjb25maWc/OiBJQ29uZmlnKSA9PiB7XG4gIHJldHVybiBheGlvZC5yZXF1ZXN0PFQ+KFxuICAgIE9iamVjdC5hc3NpZ24oe30sIHsgdXJsIH0sIGNvbmZpZywgeyBtZXRob2Q6ICdnZXQnIH0pLFxuICApO1xufTtcbmF4aW9kLnBvc3QgPSA8VCA9IGFueT4odXJsOiBzdHJpbmcsIGRhdGE/OiBEYXRhLCBjb25maWc/OiBJQ29uZmlnKSA9PiB7XG4gIHJldHVybiBheGlvZC5yZXF1ZXN0PFQ+KFxuICAgIE9iamVjdC5hc3NpZ24oe30sIHsgdXJsIH0sIGNvbmZpZywgeyBtZXRob2Q6ICdwb3N0JywgZGF0YSB9KSxcbiAgKTtcbn07XG5heGlvZC5wdXQgPSA8VCA9IGFueT4odXJsOiBzdHJpbmcsIGRhdGE/OiBEYXRhLCBjb25maWc/OiBJQ29uZmlnKSA9PiB7XG4gIHJldHVybiBheGlvZC5yZXF1ZXN0PFQ+KFxuICAgIE9iamVjdC5hc3NpZ24oe30sIHsgdXJsIH0sIGNvbmZpZywgeyBtZXRob2Q6ICdwdXQnLCBkYXRhIH0pLFxuICApO1xufTtcbmF4aW9kLmRlbGV0ZSA9IDxUID0gYW55Pih1cmw6IHN0cmluZywgZGF0YT86IERhdGEsIGNvbmZpZz86IElDb25maWcpID0+IHtcbiAgcmV0dXJuIGF4aW9kLnJlcXVlc3Q8VD4oXG4gICAgT2JqZWN0LmFzc2lnbih7fSwgeyB1cmwgfSwgY29uZmlnLCB7IG1ldGhvZDogJ2RlbGV0ZScsIGRhdGEgfSksXG4gICk7XG59O1xuYXhpb2Qub3B0aW9ucyA9IDxUID0gYW55Pih1cmw6IHN0cmluZywgZGF0YT86IERhdGEsIGNvbmZpZz86IElDb25maWcpID0+IHtcbiAgcmV0dXJuIGF4aW9kLnJlcXVlc3Q8VD4oXG4gICAgT2JqZWN0LmFzc2lnbih7fSwgeyB1cmwgfSwgY29uZmlnLCB7IG1ldGhvZDogJ29wdGlvbnMnLCBkYXRhIH0pLFxuICApO1xufTtcbmF4aW9kLmhlYWQgPSA8VCA9IGFueT4odXJsOiBzdHJpbmcsIGRhdGE/OiBEYXRhLCBjb25maWc/OiBJQ29uZmlnKSA9PiB7XG4gIHJldHVybiBheGlvZC5yZXF1ZXN0PFQ+KFxuICAgIE9iamVjdC5hc3NpZ24oe30sIHsgdXJsIH0sIGNvbmZpZywgeyBtZXRob2Q6ICdoZWFkJywgZGF0YSB9KSxcbiAgKTtcbn07XG5heGlvZC5jb25uZWN0ID0gPFQgPSBhbnk+KHVybDogc3RyaW5nLCBkYXRhPzogRGF0YSwgY29uZmlnPzogSUNvbmZpZykgPT4ge1xuICByZXR1cm4gYXhpb2QucmVxdWVzdDxUPihcbiAgICBPYmplY3QuYXNzaWduKHt9LCB7IHVybCB9LCBjb25maWcsIHsgbWV0aG9kOiAnY29ubmVjdCcsIGRhdGEgfSksXG4gICk7XG59O1xuYXhpb2QudHJhY2UgPSA8VCA9IGFueT4odXJsOiBzdHJpbmcsIGRhdGE/OiBEYXRhLCBjb25maWc/OiBJQ29uZmlnKSA9PiB7XG4gIHJldHVybiBheGlvZC5yZXF1ZXN0PFQ+KFxuICAgIE9iamVjdC5hc3NpZ24oe30sIHsgdXJsIH0sIGNvbmZpZywgeyBtZXRob2Q6ICd0cmFjZScsIGRhdGEgfSksXG4gICk7XG59O1xuYXhpb2QucGF0Y2ggPSA8VCA9IGFueT4odXJsOiBzdHJpbmcsIGRhdGE/OiBEYXRhLCBjb25maWc/OiBJQ29uZmlnKSA9PiB7XG4gIHJldHVybiBheGlvZC5yZXF1ZXN0PFQ+KFxuICAgIE9iamVjdC5hc3NpZ24oe30sIHsgdXJsIH0sIGNvbmZpZywgeyBtZXRob2Q6ICdwYXRjaCcsIGRhdGEgfSksXG4gICk7XG59O1xuXG5heGlvZC5pbnRlcmNlcHRvcnMgPSB7XG4gIHJlcXVlc3Q6IGFkZEludGVyY2VwdG9yPElBeGlvZFJlcXVlc3RJbnRlcmNlcHRvciwgSUF4aW9kUmVxdWVzdEVycm9ySW50ZXJjZXB0b3I+KCksXG4gIHJlc3BvbnNlOiBhZGRJbnRlcmNlcHRvcjxJQXhpb2RSZXNwb25zZUludGVyY2VwdG9yLCBJQXhpb2RSZXNwb25zZUVycm9ySW50ZXJjZXB0b3I+KCksXG59O1xuXG5leHBvcnQgZGVmYXVsdCBheGlvZDtcbmV4cG9ydCB7IGF4aW9kIH07XG4iLCJpbXBvcnQgYXhpb2QgZnJvbSBcImh0dHBzOi8vZGVuby5sYW5kL3gvYXhpb2QvbW9kLnRzXCI7XG5cbmFzeW5jIGZ1bmN0aW9uIGhlbGxvKCkge1xuICAgIGNvbnN0IHsgZGF0YSB9ID0gYXdhaXQgYXhpb2Q8eyBkZWxheTogc3RyaW5nIH0+KFxuICAgICAgICBcImh0dHBzOi8vcG9zdG1hbi1lY2hvLmNvbS9kZWxheS8yXCJcbiAgICAgICk7XG4gICAgICBcbiAgICBjb25zb2xlLmxvZyhkYXRhKTtcbn1cblxuaGVsbG8oKTtcbiJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFBTyxNQUFNLE9BQU8sR0FBRyxTQUFVLEdBQUcsSUFBSSxBQUFPLEVBQUU7SUFDL0MsSUFBSSxLQUFLLEFBQUM7SUFFVixJQUFJLE9BQU8sSUFBSSxDQUFDLENBQUMsQ0FBQyxLQUFLLFFBQVEsRUFBRTtRQUMvQixLQUFLLEdBQUcsSUFBSSxDQUFDLENBQUMsQ0FBQyxDQUFDO0lBQ2xCLE9BQU87UUFDTCxLQUFLLEdBQUcsRUFBRSxDQUFDLEtBQUssQ0FBQyxJQUFJLENBQUMsSUFBSSxDQUFDLENBQUM7SUFDOUIsQ0FBQztJQUVELE9BQU8sU0FBUyxDQUFDLEtBQUssQ0FBQyxDQUFDO0FBQzFCLENBQUMs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QVcsRUFBRSxNQUFnQixHQUFLO1FBQzdELE9BQU8sUUFBUSxDQUFDLE9BQU8sQ0FDckIsTUFBTSxDQUFDLE1BQU0sQ0FBQyxFQUFFLEVBQUU7WUFBRSxHQUFHO1NBQUUsRUFBRSxNQUFNLEVBQUU7WUFBRSxNQUFNLEVBQUUsS0FBSztZQUFFLElBQUk7U0FBRSxDQUFDLENBQzVELENBQUM7SUFDSixDQUFDLENBQUM7SUFDRixRQUFRLENBQUMsTUFBTSxHQUFHLENBQUMsR0FBVyxFQUFFLElBQVcsRUFBRSxNQUFnQixHQUFLO1FBQ2hFLE9BQU8sUUFBUSxDQUFDLE9BQU8sQ0FDckIsTUFBTSxDQUFDLE1BQU0sQ0FBQyxFQUFFLEVBQUU7WUFBRSxHQUFHO1NBQUUsRUFBRSxNQUFNLEVBQUU7WUFBRSxNQUFNLEVBQUUsUUFBUTtZQUFFLElBQUk7U0FBRSxDQUFDLENBQy9ELENBQUM7SUFDSixDQUFDLENBQUM7SUFDRixRQUFRLENBQUMsT0FBTyxHQUFHLENBQUMsR0FBVyxFQUFFLElBQVcsRUFBRSxNQUFnQixHQUFLO1FBQ2pFLE9BQU8sUUFBUSxDQUFDLE9BQU8sQ0FDckIsTUFBTSxDQUFDLE1BQU0sQ0FBQyxFQUFFLEVBQUU7WUFBRSxHQUFHO1NBQUUsRUFBRSxNQUFNLEVBQUU7WUFBRSxNQUFNLEVBQUUsU0FBUztZQUFFLElBQUk7U0FBRSxDQUFDLENBQ2hFLENBQUM7SUFDSixDQUFDLENBQUM7SUFDRixRQUFRLENBQUMsSUFBSSxHQUFHLENBQUMsR0FBVyxFQUFFLElBQVcsRUFBRSxNQUFnQixHQUFLO1FBQzlELE9BQU8sUUFBUSxDQUFDLE9BQU8sQ0FDckIsTUFBTSxDQUFDLE1BQU0sQ0FBQyxFQUFFLEVBQUU7WUFBRSxHQUFHO1NBQUUsRUFBRSxNQUFNLEVBQUU7WUFBRSxNQUFNLEVBQUUsTUFBTTtZQUFFLElBQUk7U0FBRSxDQUFDLENBQzdELENBQUM7SUFDSixDQUFDLENBQUM7SUFDRixRQUFRLENBQUMsT0FBTyxHQUFHLENBQUMsR0FBVyxFQUFFLElBQVcsRUFBRSxNQUFnQixHQUFLO1FBQ2pF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


  "#).unwrap();

  worker.run_event_loop(false).await?;
  Ok(())
}
