/** Copyright 2020 Alibaba Group Holding Limited.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * 	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef ANALYTICAL_ENGINE_CORE_OBJECT_GRAPH_UTILS_H_
#define ANALYTICAL_ENGINE_CORE_OBJECT_GRAPH_UTILS_H_

#include <map>
#include <memory>
#include <string>
#include <utility>

#include "grape/worker/comm_spec.h"
#include "vineyard/client/client.h"

#include "core/object/gs_object.h"
#include "core/object/i_fragment_wrapper.h"
#include "core/server/rpc_utils.h"
#include "core/utils/lib_utils.h"
#include "proto/attr_value.pb.h"

namespace gs {

typedef void LoadGraphT(
    const grape::CommSpec& comm_spec, vineyard::Client& client,
    const std::string& graph_name, const rpc::GSParams& params,
    bl::result<std::shared_ptr<IFragmentWrapper>>& fragment_wrapper);

typedef void AddVerticesToGraphT(
    vineyard::ObjectID frag_id, const grape::CommSpec& comm_spec,
    vineyard::Client& client, const std::string& graph_name,
    const rpc::GSParams& params,
    bl::result<std::shared_ptr<IFragmentWrapper>>& fragment_wrapper);

typedef void AddEdgesToGraphT(
    vineyard::ObjectID frag_id, const grape::CommSpec& comm_spec,
    vineyard::Client& client, const std::string& graph_name,
    const rpc::GSParams& params,
    bl::result<std::shared_ptr<IFragmentWrapper>>& fragment_wrapper);

typedef void ToArrowFragmentT(
    vineyard::Client& client, const grape::CommSpec& comm_spec,
    std::shared_ptr<IFragmentWrapper>& wrapper_in,
    const std::string& dst_graph_name,
    bl::result<std::shared_ptr<IFragmentWrapper>>& wrapper_out);

typedef void ToDynamicFragmentT(
    const grape::CommSpec& comm_spec,
    std::shared_ptr<IFragmentWrapper>& wrapper_in,
    const std::string& dst_graph_name,
    bl::result<std::shared_ptr<IFragmentWrapper>>& wrapper_out);

/**
 * @brief PropertyGraphUtils is a invoker of property_graph_frame library. This
 * utility provides these methods to manipulate ArrowFragment: LoadGraph,
 * ToArrowFragment and ToDynamicFragment.
 */
class PropertyGraphUtils : public GSObject {
 public:
  PropertyGraphUtils(std::string id, std::string lib_path)
      : GSObject(std::move(id), ObjectType::kPropertyGraphUtils),
        lib_path_(std::move(lib_path)),
        dl_handle_(nullptr),
        load_graph_(nullptr),
        add_vertices_to_graph_(nullptr),
        add_edges_to_graph_(nullptr),
        to_arrow_fragment_(nullptr),
        to_dynamic_fragment_(nullptr) {}

  bl::result<void> Init() {
    { BOOST_LEAF_ASSIGN(dl_handle_, open_lib(lib_path_.c_str())); }
    {
      BOOST_LEAF_AUTO(p_fun, get_func_ptr(lib_path_, dl_handle_, "LoadGraph"));
      load_graph_ = reinterpret_cast<LoadGraphT*>(p_fun);
    }
    {
      BOOST_LEAF_AUTO(
          p_fun, get_func_ptr(lib_path_, dl_handle_, "AddVerticesToGraph"));
      add_vertices_to_graph_ = reinterpret_cast<AddVerticesToGraphT*>(p_fun);
    }
    {
      BOOST_LEAF_AUTO(p_fun,
                      get_func_ptr(lib_path_, dl_handle_, "AddEdgesToGraph"));
      add_edges_to_graph_ = reinterpret_cast<AddEdgesToGraphT*>(p_fun);
    }
    {
      BOOST_LEAF_AUTO(p_fun,
                      get_func_ptr(lib_path_, dl_handle_, "ToArrowFragment"));
      to_arrow_fragment_ = reinterpret_cast<ToArrowFragmentT*>(p_fun);
    }
    {
      BOOST_LEAF_AUTO(p_fun,
                      get_func_ptr(lib_path_, dl_handle_, "ToDynamicFragment"));
      to_dynamic_fragment_ = reinterpret_cast<ToDynamicFragmentT*>(p_fun);
    }
    return {};
  }

  bl::result<std::shared_ptr<IFragmentWrapper>> LoadGraph(
      const grape::CommSpec& comm_spec, vineyard::Client& client,
      const std::string& graph_name, const rpc::GSParams& params) {
    bl::result<std::shared_ptr<IFragmentWrapper>> wrapper;

    load_graph_(comm_spec, client, graph_name, params, wrapper);
    return wrapper;
  }

  bl::result<std::shared_ptr<IFragmentWrapper>> AddVerticesToGraph(
      vineyard::ObjectID frag_id, const grape::CommSpec& comm_spec,
      vineyard::Client& client, const std::string& graph_name,
      const rpc::GSParams& params) {
    bl::result<std::shared_ptr<IFragmentWrapper>> wrapper;

    add_vertices_to_graph_(frag_id, comm_spec, client, graph_name, params,
                           wrapper);
    return wrapper;
  }

  bl::result<std::shared_ptr<IFragmentWrapper>> AddEdgesToGraph(
      vineyard::ObjectID frag_id, const grape::CommSpec& comm_spec,
      vineyard::Client& client, const std::string& graph_name,
      const rpc::GSParams& params) {
    bl::result<std::shared_ptr<IFragmentWrapper>> wrapper;

    add_edges_to_graph_(frag_id, comm_spec, client, graph_name, params,
                        wrapper);
    return wrapper;
  }

  bl::result<std::shared_ptr<IFragmentWrapper>> ToArrowFragment(
      vineyard::Client& client, const grape::CommSpec& comm_spec,
      std::shared_ptr<IFragmentWrapper>& wrapper_in,
      const std::string& dst_graph_name) {
#ifdef EXPERIMENTAL_ON
    bl::result<std::shared_ptr<IFragmentWrapper>> wrapper;

    to_arrow_fragment_(client, comm_spec, wrapper_in, dst_graph_name, wrapper);
    return wrapper;
#else
    RETURN_GS_ERROR(vineyard::ErrorCode::kUnsupportedOperationError,
                    "GS is compiled with EXPERIMENTAL_ON=OFF");
#endif
  }

  bl::result<std::shared_ptr<IFragmentWrapper>> ToDynamicFragment(
      const grape::CommSpec& comm_spec,
      std::shared_ptr<IFragmentWrapper>& wrapper_in,
      const std::string& dst_graph_name) {
#ifdef EXPERIMENTAL_ON
    bl::result<std::shared_ptr<IFragmentWrapper>> wrapper;

    to_dynamic_fragment_(comm_spec, wrapper_in, dst_graph_name, wrapper);
    return wrapper;
#else
    RETURN_GS_ERROR(vineyard::ErrorCode::kUnsupportedOperationError,
                    "GS is compiled with EXPERIMENTAL_ON=OFF");
#endif
  }

 private:
  std::string lib_path_;
  void* dl_handle_;
  LoadGraphT* load_graph_;
  AddVerticesToGraphT* add_vertices_to_graph_;
  AddEdgesToGraphT* add_edges_to_graph_;
  ToArrowFragmentT* to_arrow_fragment_;
  ToDynamicFragmentT* to_dynamic_fragment_;
};

}  // namespace gs
#endif  // ANALYTICAL_ENGINE_CORE_OBJECT_GRAPH_UTILS_H_
