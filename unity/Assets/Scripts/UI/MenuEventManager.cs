using UnityEngine;
using UnityEngine.Events;
using System.Collections;
using System.Collections.Generic;
using UnityEngine.UIElements;
using UnityEngine.SceneManagement;
using UnityEditor;
using UnityEngine.InputSystem;
using Unity.VisualScripting;
using System;
using System.IO;

public enum Menu
{
    None,
    Main,
    CreateNewTree,
    LoadTree,
    StartClam,
    LoadClam,
    Pause,
    Lock,
    Unlock,
    ResumePlay,
    IncludeHidden,
    DestroyGraph,
    DestroyTree,
    ResetTree,
    WorldInput,
    MenuInput
}

namespace Clam
{

    public class MenuEventManager : MonoBehaviour
    {
        public GameObject m_MainMenuPrefab;
        public GameObject m_CreateNewTreeMenuPrefab;
        public GameObject m_LoadTreeMenuPrefab;
        public GameObject m_PauseMenu;
        public GameObject m_InitalMenu;
        public GameObject m_ClusterSideMenu;

        public TreeStartupData m_TreeData;
        private GameObject m_CurrentMenu;

        private Dictionary<Menu, UnityEvent> m_EventDictionary;
        private static MenuEventManager m_EventManager;

        public bool m_IsPhysicsRunning = false;

        public void Start()
        {
            m_CurrentMenu = Instantiate(m_InitalMenu);
        }

        public GameObject MyInstantiate(GameObject obj)
        {
            return Instantiate(obj);
        }

        public GameObject GetCurrentMenu()
        {
            return m_CurrentMenu;
        }

        public static MenuEventManager instance
        {
            get
            {
                if (!m_EventManager)
                {
                    m_EventManager = FindObjectOfType(typeof(MenuEventManager)) as MenuEventManager;

                    if (!m_EventManager)
                    {
                        Debug.LogError("There needs to be one active EventManger script on a GameObject in your scene.");
                    }
                    else
                    {
                        m_EventManager.Init();
                    }
                }
                return m_EventManager;
            }
        }

        void Init()
        {
            if (m_EventDictionary == null)
            {
                m_EventDictionary = new Dictionary<Menu, UnityEvent>();

                StartListening(Menu.Main, SwitchToMainMenu);
                StartListening(Menu.CreateNewTree, SwitchToCreateTree);
                StartListening(Menu.LoadTree, SwitchToLoadTree);
                StartListening(Menu.StartClam, StartClam);
                StartListening(Menu.LoadClam, LoadClam);

                StartListening(Menu.Lock, LockUserInput);
                StartListening(Menu.Unlock, UnLockUserInput);
                StartListening(Menu.Pause, Pause);
                StartListening(Menu.IncludeHidden, IncludeHiddenInSelection);
                StartListening(Menu.DestroyTree, DestroyTree);

            }
        }

        public void Quit()
        {
            // save any game data here
#if UNITY_EDITOR
            // Application.Quit() does not work in the editor so
            // UnityEditor.EditorApplication.isPlaying need to be set to false to end the game
            UnityEditor.EditorApplication.isPlaying = false;
#else
        Application.Quit();
#endif
        }

        void DestroyTree()
        {
            if (m_IsPhysicsRunning)
            {
                Debug.Log("Error cannot destroy graph while physics is running");
                return;
            }

            Cakes.Tree.DestroyEdges();
        }

        void Pause()
        {
            var existingPauseMenu = FindObjectOfType(typeof(PauseMenu)) as PauseMenu;
            if (existingPauseMenu != null)
            {
                Debug.Log("already paused");
                return;
            }

            SwitchState(Menu.Unlock);
            var pauseMenu = Instantiate(m_PauseMenu);
            var resumeButton = pauseMenu.GetComponent<UIDocument>().rootVisualElement.Q<Button>("Resume");

            resumeButton.clickable.clicked += () =>
            {
                Destroy(pauseMenu);
            };
        }

        void SwitchToMainMenu()
        {
            SceneManager.LoadScene("Scenes/MainMenu");
        }
        void SwitchToCreateTree()
        {
            m_CurrentMenu = Instantiate(m_CreateNewTreeMenuPrefab);
        }
        void SwitchToLoadTree()
        {
            m_CurrentMenu = Instantiate(m_LoadTreeMenuPrefab);
        }

        void StartClam()
        {
            var doc = m_CurrentMenu.GetComponent<UIDocument>();
            string dataName = doc.rootVisualElement.Q<TextField>("DatasetInputField").value;
            // this error handling should be taken care of by the textfield (i.e int parse)
            int cardinality = int.Parse(doc.rootVisualElement.Q<TextField>("CardinalityInputField").value);
            int distanceMetric = doc.rootVisualElement.Q<DropdownField>("DistanceMetricDropdown").index + 1;// account for None
            m_TreeData.cardinality = (uint)cardinality;
            m_TreeData.dataName = dataName;
            m_TreeData.distanceMetric = (Clam.DistanceMetric)distanceMetric;
            m_TreeData.shouldLoad = false;
            m_TreeData.isExpensive = false; // change later
            SceneManager.LoadScene("Scenes/MainScene");
        }

        void LoadClam()
        {
            var doc = m_CurrentMenu.GetComponent<UIDocument>();
            string dataName = doc.rootVisualElement.Q<TextField>("LoadTreeInputField").value;
            m_TreeData.shouldLoad = true;
            try
            {
                (var metric, var isExpensive) = ParseLoadPath(dataName);

                m_TreeData.cardinality = 0;
                m_TreeData.dataName = dataName = "../data/binaries/" + dataName;
                m_TreeData.distanceMetric = metric;
                m_TreeData.isExpensive = isExpensive;
                SceneManager.LoadScene("Scenes/MainScene");

            }
            catch (Exception ex)
            {
                Debug.LogError("Invalid Cakes Load path: " + ex);
            }
        }

        private static (DistanceMetric, bool) ParseLoadPath(string input)
        {
            var parts = input.Split('_');

            if (parts.Length != 3)
            {
                throw new ArgumentException("Invalid string format");
            }

            // Parsing Metric
            if (!Enum.TryParse(parts[1], true, out DistanceMetric metric))
            {
                throw new ArgumentException($"Invalid DistanceMetric: {parts[1]}");
            }

            // Parsing IsExpensive
            if (!bool.TryParse(parts[2], out bool isExpensive))
            {
                throw new ArgumentException($"Invalid IsExpensive: {parts[2]}");
            }

            return (metric, isExpensive);
        }

        private void IncludeHiddenInSelection()
        {
            //m_CurrentMenu.GetComponent<ClusterMenu>().IncludeHiddenInSelection();
        }

        private void LockUserInput()
        {
            UnityEngine.Cursor.lockState = CursorLockMode.Locked;
            m_CurrentMenu.GetComponent<SideMenu>().SetFocusable(false);
            m_CurrentMenu.GetComponent<SideMenu>().SwitchToCameraControlMode();
            UnityEngine.Cursor.visible = false;
        }

        private void UnLockUserInput()
        {
            UnityEngine.Cursor.lockState = CursorLockMode.None;
            m_CurrentMenu.GetComponent<SideMenu>().SetFocusable(true);
            m_CurrentMenu.GetComponent<SideMenu>().SwitchToOverlayUIMode();

            UnityEngine.Cursor.visible = true;
        }

        public static void StartListening(Menu eventName, UnityAction listener)
        {
            UnityEvent thisEvent = null;
            if (instance.m_EventDictionary.TryGetValue(eventName, out thisEvent))
            {
                thisEvent.AddListener(listener);
            }
            else
            {
                thisEvent = new UnityEvent();
                thisEvent.AddListener(listener);
                instance.m_EventDictionary.Add(eventName, thisEvent);
            }
        }

        public static void StopListening(Menu eventName, UnityAction listener)
        {
            if (m_EventManager == null) return;
            UnityEvent thisEvent = null;
            if (instance.m_EventDictionary.TryGetValue(eventName, out thisEvent))
            {
                thisEvent.RemoveListener(listener);
            }
        }

        public static void SwitchState(Menu eventName)
        {
            Debug.Log("switching state to " + eventName.ToString());
            UnityEvent thisEvent = null;
            if (instance.m_EventDictionary.TryGetValue(eventName, out thisEvent))
            {
                thisEvent.Invoke();
            }
        }

        public void Update()
        {
            
        }

        public static void SwitchInputActionMap(string newMapName, PlayerInput input)
        {
            if (newMapName == input.currentActionMap.name)
            {
                return;
            }
            else
            {
                input.currentActionMap.Disable();
                input.SwitchCurrentActionMap(newMapName);

                if (newMapName == "WorldUI")
                {
                    SwitchState(Menu.Unlock);
                }
                else if (newMapName == "Player")
                {
                    SwitchState(Menu.Lock);
                }
            }
        }
    }
}
