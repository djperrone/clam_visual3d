using Clam.FFI;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.InputSystem;
using UnityEngine.UIElements;

namespace Clam
{

    public class ClamUserInput : MonoBehaviour
    {

        public PlayerInput m_PlayerInput;

        public void Start()
        {
            UnityEngine.Cursor.lockState = CursorLockMode.Locked;
            UnityEngine.Cursor.visible = false;
        }

        public void OnChangeMapToPlayer(InputValue value)
        {
            BlurFocus();

            m_PlayerInput.SwitchCurrentActionMap("Player");
            MenuEventManager.SwitchState(Menu.Lock);
        }

        public void BlurFocus()
        {
            var focusedElement = GetFocusedElement();
            if (focusedElement != null)
            {
                focusedElement.Blur();
            }
        }

        public void OnLMC()
        {
            Camera.main.ScreenToViewportPoint(Mouse.current.position.ReadValue());
            Vector3 mousePosition = Mouse.current.position.ReadValue();
            Ray ray = Camera.main.ScreenPointToRay(mousePosition);
            RaycastHit hitInfo;

            if (PointerIsUIHit(mousePosition))
            {
                return;
            }

            if (Physics.Raycast(ray.origin, ray.direction * 10, out hitInfo, Mathf.Infinity))
            {
                var selectedNode = hitInfo.collider.gameObject;
                (FFIError err, ClusterData clusterData) = NativeMethods.GetClusterData(selectedNode.GetComponent<Node>().GetId());
                //var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(selectedNode.GetComponent<Node>().GetId()));
                if (err == FFIError.Ok)
                {
                    if (!selectedNode.GetComponent<Node>().Selected)
                    {
                        MenuEventManager.instance.GetCurrentMenu().GetComponent<SideMenu>().DisplayClusterInfo(clusterData);
                    }
                    else
                    {
                        MenuEventManager.instance.GetCurrentMenu().GetComponent<SideMenu>().ClearClusterInfo();
                    }
                    selectedNode.GetComponent<Node>().ToggleSelect();
                }
                else
                {
                    Debug.LogError("wrapper was null in Create ClusterData");
                }
            }
        }

        public void OnRMC()
        {
            Camera.main.ScreenToViewportPoint(Mouse.current.position.ReadValue());
            Vector3 mousePosition = Mouse.current.position.ReadValue();
            Ray ray = Camera.main.ScreenPointToRay(mousePosition);
            RaycastHit hitInfo;

            if (PointerIsUIHit(mousePosition))
            {
                return;
            }

            if (Physics.Raycast(ray.origin, ray.direction * 10, out hitInfo, Mathf.Infinity))
            {
                var selectedNode = hitInfo.collider.gameObject;

                if (!selectedNode.GetComponent<Node>().IsLeaf()) //redundant?...
                {
                    var lid = selectedNode.GetComponent<Node>().GetLeftChildID();
                    var rid = selectedNode.GetComponent<Node>().GetRightChildID();
                    GameObject leftChild = null;
                    GameObject rightChild = null;

                    //refactor all of this nonsense later ffs
                    if (!Cakes.Tree.Contains(lid))
                    {
                        leftChild = Cakes.Tree.Add(lid);
                        leftChild.SetActive(false);

                    }
                    if (!Cakes.Tree.Contains(rid))
                    {
                        rightChild = Cakes.Tree.Add(rid);
                        rightChild.SetActive(false);
                    }

                    bool hasLeft = Cakes.Tree.GetTree().TryGetValue(lid, out leftChild);

                    bool hasRight = Cakes.Tree.GetTree().TryGetValue(rid, out rightChild);
                    // should i handle case of only one being active?
                    if (leftChild.activeSelf && rightChild.activeSelf)
                    {
                        Clam.FFI.NativeMethods.ForEachDFT(SetInactiveCallBack, leftChild.GetComponent<Node>().ID());
                        Clam.FFI.NativeMethods.ForEachDFT(SetInactiveCallBack, rightChild.GetComponent<Node>().ID());
                    }
                    else
                    {
                        leftChild.SetActive(true);
                        rightChild.SetActive(true);

                        // need to redraw parent child lines
                        (FFIError err, ClusterData clusterData) = NativeMethods.GetClusterData(selectedNode.GetComponent<Node>().GetId());

                        //var wrapper = new RustResourceWrapper<ClusterData>(ClusterData.Alloc(selectedNode.GetComponent<Node>().GetId()));
                        Clam.FFI.NativeMethods.DrawHierarchyOffsetFrom(ref clusterData, PositionUpdater);

                        //redrawing lines here after reingold call potentially
                        var springPrefab = Resources.Load("Spring") as GameObject;
                        var leftSpring = MenuEventManager.instance.MyInstantiate(springPrefab);
                        var rightSpring = MenuEventManager.instance.MyInstantiate(springPrefab);

                        leftSpring.GetComponent<Edge>().SetNodes(selectedNode, leftChild);
                        rightSpring.GetComponent<Edge>().SetNodes(selectedNode, rightChild);
                    }
                }
            }
        }

        unsafe void SetInactiveCallBack(ref FFI.ClusterData nodeData)
        {
            bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.ID_AsTuple(), out GameObject node);
            if (hasValue)
            {
                node.SetActive(false);
            }
            else
            {
                Debug.LogWarning("set inactive key not found - " + nodeData.ID_AsString());
            }
        }
        unsafe void PositionUpdater(ref Clam.FFI.ClusterData nodeData)
        {

            bool hasValue = Cakes.Tree.GetTree().TryGetValue(nodeData.ID_AsTuple(), out GameObject node);
            if (hasValue)
            {
                node.GetComponent<Node>().SetPosition(nodeData.pos.AsVector3);
            }
            else
            {
                Debug.Log("reingoldify key not found - " + nodeData.ID_AsString());
            }
        }
        void OnExit()
        {
            m_PlayerInput.SwitchCurrentActionMap("WorldUI");

            MenuEventManager.SwitchState(Menu.Pause);
        }

        public static Focusable GetFocusedElement()
        {
            EventSystem eventSystem = EventSystem.current;
            if (eventSystem == null)
            {
                return null;
            }

            GameObject selectedGameObject = eventSystem.currentSelectedGameObject;
            if (selectedGameObject == null)
            {
                return null;
            }

            PanelEventHandler panelEventHandler = selectedGameObject.GetComponent<PanelEventHandler>();
            if (panelEventHandler != null)
            {
                return panelEventHandler.panel.focusController.focusedElement;
            }

            return null;
        }

        private bool PointerIsUIHit(Vector2 position)
        {
            PointerEventData pointer = new PointerEventData(EventSystem.current);
            pointer.position = position;
            List<RaycastResult> raycastResults = new List<RaycastResult>();

            EventSystem.current.RaycastAll(pointer, raycastResults);

            if (raycastResults.Count > 0)
            {
                foreach (RaycastResult result in raycastResults)
                {
                    if (result.distance == 0 && result.isValid)
                    {
                        return true;
                    }
                }
            }

            return false;
        }
    }
}

